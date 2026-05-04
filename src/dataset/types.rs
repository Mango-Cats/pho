use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use crate::Result;
use crate::algorithms::Algorithm;
use crate::ensemble::types::EnsembleAlgorithm;

/// Unified dataset for learning workflows.
///
/// Stores input pairs, target labels, the algorithm identities used to score
/// each pair, and the precomputed per-algorithm scores.
#[derive(Debug, Clone)]
pub struct Dataset {
    pub inputs: Vec<(String, String)>,
    pub targets: Vec<f32>,
    pub algorithm_names: Vec<String>,
    pub base_scores: Vec<Vec<f32>>,
}

/// Backward-compatible alias for existing learning APIs.
pub type TrainingData = Dataset;

impl Dataset {
    fn validate_shape(&self) -> Result<()> {
        if self.inputs.len() != self.targets.len() {
            return Err(crate::Error::InvalidDatasetShape(
                "inputs and targets must have same length".to_string(),
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
        std::any::type_name_of_val(algo)
            .rsplit("::")
            .next()
            .unwrap_or("unknown")
            .to_string()
    }

    fn build<S1, S2>(algorithms: &[&dyn Algorithm], labeled_data: &[(S1, S2, f32)]) -> Result<Self>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let mut inputs = Vec::with_capacity(labeled_data.len());
        let mut targets = Vec::with_capacity(labeled_data.len());
        let mut base_scores = Vec::with_capacity(labeled_data.len());
        let algorithm_names = algorithms
            .iter()
            .map(|algo| Self::algorithm_label(*algo))
            .collect::<Vec<_>>();

        for (a, b, target) in labeled_data {
            let scores = algorithms
                .iter()
                .map(|algo| algo.similarity(a.as_ref(), b.as_ref()))
                .collect::<Result<Vec<f32>>>()?;

            inputs.push((a.as_ref().to_string(), b.as_ref().to_string()));
            targets.push(*target);
            base_scores.push(scores);
        }

        let data = Self {
            inputs,
            targets,
            algorithm_names,
            base_scores,
        };
        data.validate_shape()?;
        Ok(data)
    }

    pub fn from_slice<S1, S2>(
        algorithms: &[&dyn Algorithm],
        labeled_data: &[(S1, S2, f32)],
    ) -> Result<Self>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        Self::build(algorithms, labeled_data)
    }

    pub fn from_ensemble<S1, S2>(
        ensemble: &EnsembleAlgorithm,
        labeled_data: &[(S1, S2, f32)],
    ) -> Result<Self>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let algorithms = ensemble
            .algorithms
            .iter()
            .map(|wa| wa.algorithm.as_ref())
            .collect::<Vec<_>>();
        Self::build(&algorithms, labeled_data)
    }

    pub fn from_precomputed<S1, S2>(
        algorithm_names: Vec<String>,
        labeled_data: &[(S1, S2, f32)],
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
        let targets = labeled_data
            .iter()
            .map(|(_, _, target)| *target)
            .collect::<Vec<_>>();

        let data = Self {
            inputs,
            targets,
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
        let mut header = vec!["x_1".to_string(), "x_2".to_string(), "target".to_string()];
        header.extend(
            self.algorithm_names
                .iter()
                .enumerate()
                .map(|(i, name)| format!("score_{}_{}", i, name)),
        );
        writer.write_record(header)?;

        for ((x_1, x_2), (target, scores)) in self
            .inputs
            .iter()
            .zip(self.targets.iter().zip(self.base_scores.iter()))
        {
            let mut row = Vec::with_capacity(3 + scores.len());
            row.push(x_1.clone());
            row.push(x_2.clone());
            row.push(target.to_string());
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
        let target_array = Float32Array::from(self.targets.clone());

        let mut fields = vec![
            Field::new("x_1", DataType::Utf8, false),
            Field::new("x_2", DataType::Utf8, false),
            Field::new("target", DataType::Float32, false),
        ];

        let mut columns: Vec<ArrayRef> = vec![
            Arc::new(x1_array),
            Arc::new(x2_array),
            Arc::new(target_array),
        ];

        for (i, name) in self.algorithm_names.iter().enumerate() {
            let col_name = format!("score_{}_{}", i, name);
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
