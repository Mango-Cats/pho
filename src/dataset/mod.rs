use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::Result;
use crate::algorithms::Algorithm;
use crate::ensemble::types::EnsembleAlgorithm;

/// Dataset row with optional transcriptions for each side.
///
/// `x` and `y` are the raw forms used for storage/export, while
/// `x_transcription` and `y_transcription` are used at scoring time by
/// algorithms that require phonetic input (for example, ALINE).
///
/// Deserialization behavior:
/// - `x` and `y` are required.
/// - `label`, `x_transcription`, and `y_transcription` are optional and default to `None`
///   when the corresponding CSV column is missing.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Row {
    #[serde(alias = "x_1")]
    pub x: String,
    #[serde(alias = "x_2")]
    pub y: String,
    #[serde(default)]
    pub label: Option<f32>,
    #[serde(default)]
    pub x_transcription: Option<String>,
    #[serde(default)]
    pub y_transcription: Option<String>,
}

impl Row {
    /// Create a `Row` with the required fields `x` and `y`.
    ///
    /// Optional fields (`label`, `x_transcription`, `y_transcription`) can be
    /// added with the fluent builder: `Row::builder(x, y).label(...).build()`.
    pub fn new<S1, S2>(x: S1, y: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            x: x.into(),
            y: y.into(),
            label: None,
            x_transcription: None,
            y_transcription: None,
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
            x: x.into(),
            y: y.into(),
            label: None,
            x_transcription: None,
            y_transcription: None,
        }
    }
}

/// Fluent builder for `Row` to enable ergonomic chaining of optional fields.
pub struct RowBuilder {
    x: String,
    y: String,
    label: Option<f32>,
    x_transcription: Option<String>,
    y_transcription: Option<String>,
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
        self.x_transcription = Some(x_tr.into());
        self.y_transcription = Some(y_tr.into());
        self
    }

    /// Build the final `Row` value.
    pub fn build(self) -> Row {
        Row {
            x: self.x,
            y: self.y,
            label: self.label,
            x_transcription: self.x_transcription,
            y_transcription: self.y_transcription,
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
        algorithm: &dyn Algorithm,
        row_index: usize,
    ) -> Result<(&'a str, &'a str)> {
        if !algorithm.requires_transcription() {
            return Ok((self.x.as_str(), self.y.as_str()));
        }

        match (
            self.x_transcription.as_deref(),
            self.y_transcription.as_deref(),
        ) {
            (Some(x_tr), Some(y_tr)) => Ok((x_tr, y_tr)),
            _ => Err(crate::Error::MissingTranscription {
                algorithm: algorithm.name().to_string(),
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
                    let (left, right) = row.pair_for(*algo, row_index)?;
                    algo.similarity(left, right)
                })
                .collect::<Result<Vec<f32>>>()?;

            inputs.push((row.x.clone(), row.y.clone()));
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
        let algorithms = ensemble
            .algorithms
            .iter()
            .map(|wa| wa.algorithm.as_ref())
            .collect::<Vec<_>>();
        Self::build_from_rows(&algorithms, labeled_data)
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
        let mut header = vec!["x_1".to_string(), "x_2".to_string(), "label".to_string()];
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
            Field::new("x_1", DataType::Utf8, false),
            Field::new("x_2", DataType::Utf8, false),
            Field::new("label", DataType::Float32, true),
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
}
