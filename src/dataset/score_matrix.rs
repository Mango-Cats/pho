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
}

impl ScoreMatrix {
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

        Ok(())
    }

    fn algorithm_label(algo: &dyn Algorithm) -> String {
        algo.name().to_string()
    }

    fn weighted_algorithm_label(weighted: &WeightedFunction) -> String {
        format!("{}_{}", weighted.name(), weighted.weight)
    }

    fn build_from_rows(
        algorithms: &[&dyn Algorithm],
        labeled_data: &[Row],
        show_progress: bool,
    ) -> Result<Self> {
        let algorithm_names = algorithms
            .iter()
            .map(|algo| Self::algorithm_label(*algo))
            .collect::<Vec<_>>();

        let pb = if show_progress {
            let pb = ProgressBar::new(labeled_data.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} rows ({eta})")
                    .expect("valid template"),
            );
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
        show_progress: bool,
    ) -> Result<Self> {
        let algorithms = algorithms
            .iter()
            .map(|algo| algo.as_ref())
            .collect::<Vec<_>>();
        Self::build_from_rows(&algorithms, labeled_data, show_progress)
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

        let pb = if show_progress {
            let pb = ProgressBar::new(labeled_data.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} rows ({eta})")
                    .expect("valid template"),
            );
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
    ) -> Result<Self>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let inputs = labeled_data
            .iter()
            .map(|(a, b, _)| (a.as_ref().to_string(), b.as_ref().to_string()))
            .collect::<Vec<_>>();
        let labels = labeled_data
            .iter()
            .map(|(_, _, label)| *label)
            .collect::<Vec<_>>();

        let data = Self {
            inputs,
            labels,
            algorithm_names,
            base_scores,
        };
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
            _ => Err(crate::Error::InvalidExtension(file_name.to_string())),
        }
    }

    fn export_csv(&self, file_name: &str) -> Result<()> {
        use csv::Writer;

        let mut writer = Writer::from_path(file_name)?;
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
        use arrow::array::{ArrayRef, Float32Array, StringArray};
        use arrow::datatypes::{DataType, Field, Schema};
        use arrow::ipc::writer::FileWriter;
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
}
