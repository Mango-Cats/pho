use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::Result;
use crate::algorithms::Algorithm;
use crate::ensemble::types::EnsembleAlgorithm;
use crate::ensemble::weighted_function::WeightedFunction;

/// Dataset row with optional transcriptions for each side.
///
/// `x_1` and `x_2` are the raw forms used for storage/export, while
/// `x_transcription` and `y_transcription` are used at scoring time by
/// algorithms that require phonetic input (for example, ALINE).
///
/// Deserialization behavior:
/// - `x` and `y` are required.
/// - `label`, `x_transcription`, and `y_transcription` are optional and default to `None`
///   when the corresponding CSV column is missing.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Row {
    #[serde(alias = "x", alias = "x_1")]
    pub x_1: String,
    #[serde(alias = "y", alias = "x_2")]
    pub x_2: String,
    #[serde(default)]
    pub label: Option<f32>,
    #[serde(default)]
    pub t_1: Option<String>,
    #[serde(default)]
    pub t_2: Option<String>,
}

/// Fluent builder for `Row` to enable ergonomic chaining of optional fields.
pub struct RowBuilder {
    x_1: String,
    x_2: String,
    label: Option<f32>,
    t_1: Option<String>,
    t_2: Option<String>,
}

impl Row {
    pub const COL_X_1: &'static str = "x_1";
    pub const COL_X_2: &'static str = "x_2";
    pub const COL_LABEL: &'static str = "label";
    pub const COL_T_1: &'static str = "t_1";
    pub const COL_T_2: &'static str = "t_2";

    /// Create a `Row` with the required fields `x` and `y`.
    ///
    /// Optional fields (`label`, `t_1`, `t_2`) can be
    /// added with the fluent builder: `Row::builder(x, y).label(...).build()`.
    pub fn new<S1, S2>(x: S1, y: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            x_1: x.into(),
            x_2: y.into(),
            label: None,
            t_1: None,
            t_2: None,
        }
    }

    /// Start a fluent builder for `Row`.
    ///
    /// Example: `Row::builder("a", "b").label(0.5).transcriptions("x", "y").build()`
    pub fn builder<S1, S2>(x: S1, y: S2) -> RowBuilder
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        RowBuilder {
            x_1: x.into(),
            x_2: y.into(),
            label: None,
            t_1: None,
            t_2: None,
        }
    }
}

impl RowBuilder {
    /// Set the optional label (target) for the row.
    pub fn label(mut self, value: f32) -> Self {
        self.label = Some(value);
        self
    }

    /// Set transcriptions for x and y.
    pub fn transcriptions<T1, T2>(mut self, x_tr: T1, y_tr: T2) -> Self
    where
        T1: Into<String>,
        T2: Into<String>,
    {
        self.t_1 = Some(x_tr.into());
        self.t_2 = Some(y_tr.into());
        self
    }

    /// Build the final `Row` value.
    pub fn build(self) -> Row {
        Row {
            x_1: self.x_1,
            x_2: self.x_2,
            label: self.label,
            t_1: self.t_1,
            t_2: self.t_2,
        }
    }
}

impl From<RowBuilder> for Row {
    fn from(b: RowBuilder) -> Self {
        b.build()
    }
}
impl Row {
    fn pair_for<'a>(
        &'a self,
        requires_transcription: bool,
        algorithm_name: &str,
        row_index: usize,
    ) -> Result<(&'a str, &'a str)> {
        if !requires_transcription {
            return Ok((self.x_1.as_str(), self.x_2.as_str()));
        }

        match (self.t_1.as_deref(), self.t_2.as_deref()) {
            (Some(x_tr), Some(y_tr)) => Ok((x_tr, y_tr)),
            _ => Err(crate::Error::MissingTranscription {
                algorithm: algorithm_name.to_string(),
                row_index,
            }),
        }
    }
}

/// Unified dataset for learning workflows.
///
/// Stores input pairs, optional labels, the algorithm identities used to score
/// each pair, and the precomputed per-algorithm scores.
#[derive(Debug, Clone)]
pub struct Dataset {
    pub inputs: Vec<(String, String)>,
    pub labels: Vec<Option<f32>>,
    pub algorithm_names: Vec<String>,
    pub base_scores: Vec<Vec<f32>>,
}

impl Dataset {
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

    fn build_from_rows(algorithms: &[&dyn Algorithm], labeled_data: &[Row]) -> Result<Self> {
        let mut inputs = Vec::with_capacity(labeled_data.len());
        let mut labels = Vec::with_capacity(labeled_data.len());
        let mut base_scores = Vec::with_capacity(labeled_data.len());
        let algorithm_names = algorithms
            .iter()
            .map(|algo| Self::algorithm_label(*algo))
            .collect::<Vec<_>>();

        for (row_index, row) in labeled_data.iter().enumerate() {
            let scores = algorithms
                .iter()
                .map(|algo| {
                    let (left, right) =
                        row.pair_for(algo.requires_transcription(), algo.name(), row_index)?;
                    algo.similarity(left, right)
                })
                .collect::<Result<Vec<f32>>>()?;

            inputs.push((row.x_1.clone(), row.x_2.clone()));
            labels.push(row.label);
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
    pub fn from_slice(algorithms: Vec<Box<dyn Algorithm>>, labeled_data: &[Row]) -> Result<Self> {
        let algorithms = algorithms
            .iter()
            .map(|algo| algo.as_ref())
            .collect::<Vec<_>>();
        Self::build_from_rows(&algorithms, labeled_data)
    }

    /// Build a dataset from [`Row`] values using the algorithms contained in
    /// an ensemble.
    ///
    /// Input form and label behavior are the same as [`Self::from_slice`].
    pub fn from_ensemble(ensemble: &EnsembleAlgorithm, labeled_data: &[Row]) -> Result<Self> {
        let mut inputs = Vec::with_capacity(labeled_data.len());
        let mut labels = Vec::with_capacity(labeled_data.len());
        let mut base_scores = Vec::with_capacity(labeled_data.len());

        let mut algorithm_names = Vec::with_capacity(1 + ensemble.algorithms.len());
        algorithm_names.push("ensemble".to_string());
        algorithm_names.extend(
            ensemble
                .algorithms
                .iter()
                .map(Self::weighted_algorithm_label),
        );

        for (row_index, row) in labeled_data.iter().enumerate() {
            let mut weighted_sum = 0.0f32;
            let mut total_weight = 0.0f32;

            let component_scores = ensemble
                .algorithms
                .iter()
                .map(|weighted| {
                    let (left, right) = row.pair_for(
                        weighted.requires_transcription(),
                        weighted.name(),
                        row_index,
                    )?;
                    let score = weighted.score(left, right)?;

                    if weighted.weight != 0.0 {
                        weighted_sum += score * weighted.weight;
                        total_weight += weighted.weight.abs();
                    }

                    Ok(score)
                })
                .collect::<Result<Vec<f32>>>()?;

            let ensemble_score = if total_weight == 0.0 {
                0.0
            } else {
                (weighted_sum / total_weight).clamp(0.0, 1.0)
            };

            let mut scores = Vec::with_capacity(1 + component_scores.len());
            scores.push(ensemble_score);
            scores.extend(component_scores);

            inputs.push((row.x_1.clone(), row.x_2.clone()));
            labels.push(row.label);
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

#[cfg(test)]
mod tests {
    use super::{Dataset, Row};
    use crate::ensemble::config::EnsembleConfig;
    use crate::ensemble::types::EnsembleAlgorithm;
    use crate::ensemble::weighted_function::WeightedFunction;
    use crate::{algorithms::Algorithm, error::Result};

    struct RequiresTranscription;

    impl Algorithm for RequiresTranscription {
        fn similarity(&self, x: &str, y: &str) -> Result<f32> {
            if x.is_empty() && y.is_empty() {
                return Ok(1.0);
            }
            Ok(0.5)
        }

        fn requires_transcription(&self) -> bool {
            true
        }

        fn name(&self) -> &'static str {
            "RequiresTranscription"
        }
    }

    #[test]
    fn from_slice_rejects_transcription_required_algorithm() {
        let algorithms: Vec<Box<dyn Algorithm>> = vec![Box::new(RequiresTranscription)];
        let rows = vec![Row::builder("a", "b").label(0.5f32).build()];

        let result = Dataset::from_slice(algorithms, &rows);
        assert!(result.is_err());
    }

    #[test]
    fn from_slice_accepts_transcription_required_algorithm() {
        let algorithms: Vec<Box<dyn Algorithm>> = vec![Box::new(RequiresTranscription)];
        let rows = vec![
            Row::builder("word", "ward")
                .label(0.5)
                .transcriptions("wɜd", "wɔɹd")
                .build(),
        ];

        let result = Dataset::from_slice(algorithms, &rows);
        assert!(result.is_ok());
    }

    #[test]
    fn from_slice_allows_missing_label() {
        let algorithms: Vec<Box<dyn Algorithm>> = vec![Box::new(RequiresTranscription)];
        let rows = vec![
            Row::builder("word", "ward")
                .transcriptions("wɜd", "wɔɹd")
                .build(),
        ];

        let result = Dataset::from_slice(algorithms, &rows);
        assert!(result.is_ok());
        let dataset = result.unwrap();
        assert_eq!(dataset.labels, vec![None]);
    }

    #[test]
    fn from_ensemble_includes_ensemble_and_weighted_component_scores() {
        let ensemble = EnsembleAlgorithm::try_new(
            vec![
                WeightedFunction::from_function("aline", 0.8, false, |_x, _y| Ok(0.0)),
                WeightedFunction::from_function("bisim", 0.1, false, |_x, _y| Ok(0.1)),
                WeightedFunction::from_function("editex", 0.1, false, |_x, _y| Ok(0.2)),
            ],
            EnsembleConfig::Linear,
        )
        .expect("valid ensemble");

        let rows = vec![Row::builder("drug_a", "drug_b").label(0.0).build()];
        let dataset = Dataset::from_ensemble(&ensemble, &rows).expect("dataset from ensemble");

        assert_eq!(
            dataset.algorithm_names,
            vec!["ensemble", "aline_0.8", "bisim_0.1", "editex_0.1"]
        );

        let scores = &dataset.base_scores[0];
        assert_eq!(scores.len(), 4);
        assert!((scores[0] - 0.03).abs() < 1e-6);
        assert!((scores[1] - 0.0).abs() < 1e-6);
        assert!((scores[2] - 0.1).abs() < 1e-6);
        assert!((scores[3] - 0.2).abs() < 1e-6);
    }
}
