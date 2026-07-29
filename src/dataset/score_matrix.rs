use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use crate::Result;
use crate::algorithms::Algorithm;
use crate::dataset::row::Row;
use crate::ensemble::types::EnsembleAlgorithm;
use crate::ensemble::weighted_function::WeightedFunction;

/// Unified dataset for learning workflows.
///
/// Stores input pairs, optional labels, the algorithm identities used to score
/// each pair, and the precomputed per-algorithm scores.
#[derive(Debug, Clone)]
pub struct ScoreMatrix {
    pub inputs: Vec<(String, String)>,
    pub labels: Vec<Option<f32>>,
    pub algorithm_names: Vec<String>,
    pub base_scores: Vec<Vec<f32>>,
    pub passthrough_headers: Vec<String>,
    pub passthrough_rows: Vec<Vec<String>>,
}

impl ScoreMatrix {
    const PREFIX_ALGORITHM_NAME: &'static str = "Prefix";
    const COMMON_PREFIX_RATIO_NAME: &'static str = "common_prefix_ratio";
    const WORD_LEVEL_FEATURES: [&'static str; 12] = [
        "len_x1",
        "len_x2",
        "len_min",
        "len_max",
        "len_diff",
        "len_ratio",
        "common_prefix_len",
        "common_prefix_ratio",
        "common_suffix_len",
        "common_suffix_ratio",
        "first_mismatch_pos",
        "first_char_match",
    ];

    fn validate_shape(&self) -> Result<()> {
        if self.inputs.len() != self.labels.len() {
            return Err(crate::Error::InvalidDatasetShape(
                "inputs and labels must have same length".to_string(),
            ));
        }

        if self.inputs.len() != self.base_scores.len() {
            return Err(crate::Error::InvalidDatasetShape(
                "inputs and base_scores must have same length".to_string(),
            ));
        }

        let expected_width = self.algorithm_names.len();
        if self
            .base_scores
            .iter()
            .any(|row| row.len() != expected_width)
        {
            return Err(crate::Error::InvalidDatasetShape(
                "every base_scores row must match algorithm count".to_string(),
            ));
        }

        if !self.passthrough_rows.is_empty() {
            if self.passthrough_rows.len() != self.inputs.len() {
                return Err(crate::Error::InvalidDatasetShape(
                    "passthrough rows must match input count".to_string(),
                ));
            }

            let passthrough_width = self.passthrough_headers.len();
            if self
                .passthrough_rows
                .iter()
                .any(|row| row.len() != passthrough_width)
            {
                return Err(crate::Error::InvalidDatasetShape(
                    "every passthrough row must match passthrough header count".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Attach pass-through columns preserved from the original input.
    ///
    /// `headers` names the columns and `rows` holds one string cell vector per
    /// dataset row, in the same order as the rows used to build this matrix.
    /// When present, CSV export emits these columns verbatim followed by the
    /// computed feature columns. Returns an error if the row count or any row's
    /// width is inconsistent.
    pub fn with_passthrough(
        mut self,
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    ) -> Result<Self> {
        self.passthrough_headers = headers;
        self.passthrough_rows = rows;
        self.validate_shape()?;
        Ok(self)
    }

    fn weighted_algorithm_label(weighted: &WeightedFunction) -> String {
        format!("{}_{}", weighted.name(), weighted.weight)
    }

    fn normalized_algorithm_label(name: &str, include_word_level_features: bool) -> String {
        if include_word_level_features && name == Self::PREFIX_ALGORITHM_NAME {
            Self::COMMON_PREFIX_RATIO_NAME.to_string()
        } else {
            name.to_string()
        }
    }

    fn common_prefix_len(left: &str, right: &str) -> usize {
        left.chars()
            .zip(right.chars())
            .take_while(|(left_char, right_char)| left_char == right_char)
            .count()
    }

    fn common_suffix_len(left: &str, right: &str) -> usize {
        left.chars()
            .rev()
            .zip(right.chars().rev())
            .take_while(|(left_char, right_char)| left_char == right_char)
            .count()
    }

    fn word_level_features(existing_names: &[String]) -> Vec<&'static str> {
        Self::WORD_LEVEL_FEATURES
            .iter()
            .copied()
            .filter(|name| !existing_names.iter().any(|existing| existing == name))
            .collect()
    }

    fn length_feature_values(left: &str, right: &str, feature_names: &[&'static str]) -> Vec<f32> {
        let left_len = left.chars().count();
        let right_len = right.chars().count();
        let max_len = left_len.max(right_len) as f32;
        let min_len = left_len.min(right_len) as f32;
        let prefix_len = Self::common_prefix_len(left, right) as f32;
        let suffix_len = Self::common_suffix_len(left, right) as f32;
        let length_diff = left_len.abs_diff(right_len) as f32;

        feature_names
            .iter()
            .map(|name| match *name {
                "len_x1" => left_len as f32,
                "len_x2" => right_len as f32,
                "len_min" => min_len,
                "len_max" => max_len,
                "len_diff" => length_diff,
                "len_ratio" => {
                    if max_len == 0.0 {
                        1.0
                    } else {
                        (min_len / max_len).clamp(0.0, 1.0)
                    }
                }
                "common_prefix_len" => prefix_len,
                "common_prefix_ratio" => {
                    if max_len == 0.0 {
                        1.0
                    } else {
                        (prefix_len / max_len).clamp(0.0, 1.0)
                    }
                }
                "common_suffix_len" => suffix_len,
                "common_suffix_ratio" => {
                    if max_len == 0.0 {
                        1.0
                    } else {
                        (suffix_len / max_len).clamp(0.0, 1.0)
                    }
                }
                // The index of the first mismatching character equals the
                // common prefix length; normalizing by min_len (rather than
                // max_len, as common_prefix_ratio does) means a string that
                // is a strict prefix of the other still reports full
                // agreement (1.0) over the range where a mismatch could
                // possibly occur.
                "first_mismatch_pos" => {
                    if min_len == 0.0 {
                        1.0
                    } else {
                        (prefix_len / min_len).clamp(0.0, 1.0)
                    }
                }
                "first_char_match" => {
                    if (left_len == 0 && right_len == 0) || prefix_len >= 1.0 {
                        1.0
                    } else {
                        0.0
                    }
                }
                _ => unreachable!("unknown length feature name"),
            })
            .collect()
    }

    fn append_length_features(
        scores: &mut Vec<f32>,
        left: &str,
        right: &str,
        feature_names: &[&'static str],
    ) {
        scores.extend(Self::length_feature_values(left, right, feature_names));
    }

    fn build_from_rows(
        algorithms: &[&dyn Algorithm],
        column_names: &[String],
        labeled_data: &[Row],
        include_word_level_features: bool,
        show_progress: bool,
    ) -> Result<Self> {
        let mut algorithm_names = column_names.to_vec();

        let word_level_features = if include_word_level_features {
            Self::word_level_features(&algorithm_names)
        } else {
            Vec::new()
        };

        algorithm_names.extend(word_level_features.iter().map(|name| name.to_string()));

        let pb = if show_progress {
            let pb = ProgressBar::new(labeled_data.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} rows ({eta})")
                    .expect("valid template"),
            );
            pb.set_position(0);

            Some(pb)
        } else {
            None
        };

        // Parallelize row-level computation using rayon
        let row_results: Result<Vec<_>> = labeled_data
            .par_iter()
            .enumerate()
            .map(|(row_index, row)| {
                let scores = algorithms
                    .iter()
                    .map(|algo| {
                        let (left, right) =
                            row.pair_for(algo.requires_transcription(), algo.name(), row_index)?;
                        algo.similarity(left, right)
                    })
                    .collect::<Result<Vec<f32>>>()?;

                let mut scores = scores;
                if include_word_level_features {
                    Self::append_length_features(
                        &mut scores,
                        row.x_1.as_str(),
                        row.x_2.as_str(),
                        &word_level_features,
                    );
                }

                if let Some(pb) = pb.as_ref() {
                    pb.inc(1);
                }

                Ok((row.x_1.clone(), row.x_2.clone(), row.label, scores))
            })
            .collect();

        let results = row_results?;
        if let Some(pb) = pb.as_ref() {
            pb.set_position(labeled_data.len() as u64);
            pb.finish_with_message("Dataset precomputation complete");
        }

        let mut inputs = Vec::with_capacity(results.len());
        let mut labels = Vec::with_capacity(results.len());
        let mut base_scores = Vec::with_capacity(results.len());

        for (x_1, x_2, label, scores) in results {
            inputs.push((x_1, x_2));
            labels.push(label);
            base_scores.push(scores);
        }

        let data = Self {
            inputs,
            labels,
            algorithm_names,
            base_scores,
            passthrough_headers: Vec::new(),
            passthrough_rows: Vec::new(),
        };
        data.validate_shape()?;
        Ok(data)
    }

    /// Build a dataset from [`Row`] values and a list of algorithms.
    ///
    /// Input form is inferred per algorithm:
    /// - raw `x`/`y` for algorithms that do not require transcriptions
    /// - `x_transcription`/`y_transcription` for algorithms that do
    ///
    /// Labels are optional and are stored as-is in `Dataset.labels`.
    ///
    /// If an algorithm requires transcriptions, both transcription fields must be present for
    /// each row; otherwise this returns `Error::MissingTranscription`.
    ///
    /// If `show_progress` is true, a progress bar will be displayed during precomputation.
    pub fn from_slice(
        algorithms: Vec<Box<dyn Algorithm>>,
        labeled_data: &[Row],
        include_word_level_features: bool,
        show_progress: bool,
    ) -> Result<Self> {
        let algorithms = algorithms
            .iter()
            .map(|algo| algo.as_ref())
            .collect::<Vec<_>>();
        let column_names = algorithms
            .iter()
            .map(|algo| Self::normalized_algorithm_label(algo.name(), include_word_level_features))
            .collect::<Vec<_>>();
        Self::build_from_rows(
            &algorithms,
            &column_names,
            labeled_data,
            include_word_level_features,
            show_progress,
        )
    }

    /// Build a dataset from explicitly named algorithms.
    ///
    /// Identical to [`Self::from_slice`], except each output feature column is
    /// named by the caller-provided string instead of the algorithm's own
    /// `name()`. This lets, for example, two configs of the same algorithm live
    /// in one dataset under distinct column names (e.g. one column per config
    /// file, named after the file).
    pub fn from_named(
        algorithms: Vec<(String, Box<dyn Algorithm>)>,
        labeled_data: &[Row],
        include_word_level_features: bool,
        show_progress: bool,
    ) -> Result<Self> {
        let column_names = algorithms
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();
        let algorithms = algorithms
            .iter()
            .map(|(_, algo)| algo.as_ref())
            .collect::<Vec<_>>();
        Self::build_from_rows(
            &algorithms,
            &column_names,
            labeled_data,
            include_word_level_features,
            show_progress,
        )
    }

    /// Build a dataset from [`Row`] values using the algorithms contained in
    /// an ensemble.
    ///
    /// Input form and label behavior are the same as [`Self::from_slice`].
    ///
    /// If `show_progress` is true, a progress bar will be displayed during precomputation.
    pub fn from_ensemble(
        ensemble: &EnsembleAlgorithm,
        labeled_data: &[Row],
        include_word_level_features: bool,
        show_progress: bool,
    ) -> Result<Self> {
        let mut algorithm_names = Vec::with_capacity(1 + ensemble.algorithms.len());
        algorithm_names.push("ensemble".to_string());
        algorithm_names.extend(
            ensemble
                .algorithms
                .iter()
                .map(Self::weighted_algorithm_label),
        );

        algorithm_names = algorithm_names
            .into_iter()
            .map(|name| Self::normalized_algorithm_label(&name, include_word_level_features))
            .collect::<Vec<_>>();

        let word_level_features = if include_word_level_features {
            Self::word_level_features(&algorithm_names)
        } else {
            Vec::new()
        };

        algorithm_names.extend(word_level_features.iter().map(|name| name.to_string()));

        let pb = if show_progress {
            let pb = ProgressBar::new(labeled_data.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} rows ({eta})")
                    .expect("valid template"),
            );
            pb.set_position(0);
            Some(pb)
        } else {
            None
        };

        // Parallelize row-level computation using rayon
        let row_results: Result<Vec<_>> = labeled_data
            .par_iter()
            .enumerate()
            .map(|(row_index, row)| {
                // Compute scores for all weighted functions
                let component_scores = ensemble
                    .algorithms
                    .iter()
                    .map(|weighted| {
                        let (left, right) = row.pair_for(
                            weighted.requires_transcription(),
                            weighted.name(),
                            row_index,
                        )?;
                        weighted.score(left, right)
                    })
                    .collect::<Result<Vec<f32>>>()?;

                // Compute weighted ensemble score from components
                let mut weighted_sum = 0.0f32;
                let mut total_weight = 0.0f32;
                for (score, weighted) in component_scores.iter().zip(ensemble.algorithms.iter()) {
                    if weighted.weight != 0.0 {
                        weighted_sum += score * weighted.weight;
                        total_weight += weighted.weight.abs();
                    }
                }

                let ensemble_score = if total_weight == 0.0 {
                    0.0
                } else {
                    (weighted_sum / total_weight).clamp(0.0, 1.0)
                };

                let mut scores = Vec::with_capacity(1 + component_scores.len());
                scores.push(ensemble_score);
                scores.extend(component_scores);

                if include_word_level_features {
                    Self::append_length_features(
                        &mut scores,
                        row.x_1.as_str(),
                        row.x_2.as_str(),
                        &word_level_features,
                    );
                }

                Ok((row.x_1.clone(), row.x_2.clone(), row.label, scores))
            })
            .collect();

        let results = row_results?;
        if let Some(pb) = pb.as_ref() {
            pb.set_position(labeled_data.len() as u64);
            pb.finish_with_message("Dataset precomputation complete");
        }

        let mut inputs = Vec::with_capacity(results.len());
        let mut labels = Vec::with_capacity(results.len());
        let mut base_scores = Vec::with_capacity(results.len());

        for (x_1, x_2, label, scores) in results {
            inputs.push((x_1, x_2));
            labels.push(label);
            base_scores.push(scores);
        }

        let data = Self {
            inputs,
            labels,
            algorithm_names,
            base_scores,
            passthrough_headers: Vec::new(),
            passthrough_rows: Vec::new(),
        };
        data.validate_shape()?;
        Ok(data)
    }

    /// Build a dataset from precomputed algorithm scores.
    ///
    /// Use this when you already computed base similarity values externally and
    /// only need the unified `Dataset` container for training or export.
    ///
    /// `algorithm_names.len()` must match every `base_scores[row].len()`, and
    /// `labeled_data.len()` must match `base_scores.len()`.
    pub fn from_precomputed<S1, S2>(
        algorithm_names: Vec<String>,
        labeled_data: &[(S1, S2, Option<f32>)],
        base_scores: Vec<Vec<f32>>,
        include_word_level_features: bool,
    ) -> Result<Self>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let mut algorithm_names = algorithm_names
            .into_iter()
            .map(|name| Self::normalized_algorithm_label(&name, include_word_level_features))
            .collect::<Vec<_>>();

        let word_level_features = if include_word_level_features {
            Self::word_level_features(&algorithm_names)
        } else {
            Vec::new()
        };

        algorithm_names.extend(word_level_features.iter().map(|name| name.to_string()));

        let pb = ProgressBar::new(labeled_data.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} rows ({eta})")
                .expect("valid template"),
        );
        pb.set_position(0);

        let inputs = labeled_data
            .iter()
            .enumerate()
            .map(|(_, (a, b, _))| {
                pb.inc(1);
                (a.as_ref().to_string(), b.as_ref().to_string())
            })
            .collect::<Vec<_>>();

        pb.set_position(0);
        let labels = labeled_data
            .iter()
            .enumerate()
            .map(|(_, (_, _, label))| {
                pb.inc(1);
                *label
            })
            .collect::<Vec<_>>();

        let mut base_scores = base_scores;
        if include_word_level_features {
            for ((left, right, _), scores) in labeled_data.iter().zip(base_scores.iter_mut()) {
                Self::append_length_features(
                    scores,
                    left.as_ref(),
                    right.as_ref(),
                    &word_level_features,
                );
            }
        }

        let data = Self {
            inputs,
            labels,
            algorithm_names,
            base_scores,
            passthrough_headers: Vec::new(),
            passthrough_rows: Vec::new(),
        };
        pb.set_position(labeled_data.len() as u64);
        pb.finish_with_message("Precomputed dataset loaded");
        data.validate_shape()?;
        Ok(data)
    }

    /// Exports the given dataset and infers how to export it from
    /// the file extension provided.
    pub fn export(&self, file_name: &str) -> Result<()> {
        let path = Path::new(file_name);

        match path.extension().and_then(|e| e.to_str()) {
            Some("csv") => self.export_csv(file_name),
            Some("arrow") | Some("ipc") => self.export_arrow(file_name),
            Some("parquet") => self.export_parquet(file_name),
            _ => Err(crate::Error::InvalidExtension(file_name.to_string())),
        }
    }

    fn build_record_batch(
        &self,
    ) -> Result<(
        Arc<arrow::datatypes::Schema>,
        arrow::record_batch::RecordBatch,
    )> {
        use arrow::array::{ArrayRef, Float32Array, StringArray};
        use arrow::datatypes::{DataType, Field, Schema};
        use arrow::record_batch::RecordBatch;

        let x1_array = StringArray::from(
            self.inputs
                .iter()
                .map(|(x_1, _)| x_1.as_str())
                .collect::<Vec<_>>(),
        );
        let x2_array = StringArray::from(
            self.inputs
                .iter()
                .map(|(_, x_2)| x_2.as_str())
                .collect::<Vec<_>>(),
        );
        let label_array = Float32Array::from(self.labels.clone());

        let mut fields = vec![
            Field::new(Row::COL_X_1, DataType::Utf8, false),
            Field::new(Row::COL_X_2, DataType::Utf8, false),
            Field::new(Row::COL_LABEL, DataType::Float32, true),
        ];

        let mut columns: Vec<ArrayRef> = vec![
            Arc::new(x1_array),
            Arc::new(x2_array),
            Arc::new(label_array),
        ];

        for (i, name) in self.algorithm_names.iter().enumerate() {
            let col_name = format!("{}", name);
            fields.push(Field::new(&col_name, DataType::Float32, false));
            let score_col = Float32Array::from(
                self.base_scores
                    .iter()
                    .map(|row| row[i])
                    .collect::<Vec<_>>(),
            );
            columns.push(Arc::new(score_col));
        }

        let schema = Arc::new(Schema::new(fields));
        let batch = RecordBatch::try_new(schema.clone(), columns)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        Ok((schema, batch))
    }

    fn export_csv(&self, file_name: &str) -> Result<()> {
        use csv::Writer;

        let mut writer = Writer::from_path(file_name)?;

        // Pass-through mode: re-emit every original input column, then append
        // the computed feature columns. Rows line up with `base_scores` by index.
        if !self.passthrough_headers.is_empty() {
            let mut header = self.passthrough_headers.clone();
            header.extend(self.algorithm_names.iter().cloned());
            writer.write_record(header)?;

            for (original, scores) in self.passthrough_rows.iter().zip(self.base_scores.iter()) {
                let mut row = original.clone();
                row.extend(scores.iter().map(|score| score.to_string()));
                writer.write_record(row)?;
            }

            writer.flush()?;
            return Ok(());
        }

        let mut header = vec![
            Row::COL_X_1.to_string(),
            Row::COL_X_2.to_string(),
            Row::COL_LABEL.to_string(),
        ];
        header.extend(
            self.algorithm_names
                .iter()
                .enumerate()
                .map(|(_, name)| format!("{}", name)),
        );
        writer.write_record(header)?;

        for ((x_1, x_2), (label, scores)) in self
            .inputs
            .iter()
            .zip(self.labels.iter().zip(self.base_scores.iter()))
        {
            let mut row = Vec::with_capacity(3 + scores.len());
            row.push(x_1.clone());
            row.push(x_2.clone());
            row.push(label.map(|t| t.to_string()).unwrap_or_default());
            row.extend(scores.iter().map(|score| score.to_string()));
            writer.write_record(row)?;
        }

        writer.flush()?;
        Ok(())
    }

    fn export_arrow(&self, file_name: &str) -> Result<()> {
        use arrow::ipc::writer::FileWriter;
        let (schema, batch) = self.build_record_batch()?;

        let file = File::create(file_name)?;
        let mut writer = FileWriter::try_new(file, &schema)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        writer
            .write(&batch)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        writer
            .finish()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        Ok(())
    }

    fn export_parquet(&self, file_name: &str) -> Result<()> {
        use parquet::arrow::ArrowWriter;

        let (schema, batch) = self.build_record_batch()?;
        let file = File::create(file_name)?;
        let mut writer = ArrowWriter::try_new(file, schema, None)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        writer
            .write(&batch)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        writer
            .close()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        Ok(())
    }
}
