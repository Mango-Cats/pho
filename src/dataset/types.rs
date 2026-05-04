use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use crate::Result;

/// You must provide a stable, serializable representation.
pub trait Algorithm {
    fn name(&self) -> &str;
}

pub struct Dataset<'a, const SIZE: usize> {
    pub(crate) x_1: [&'a str; SIZE],
    pub(crate) x_2: [&'a str; SIZE],
    pub(crate) algorithms: [Box<dyn Algorithm>; SIZE],
    pub(crate) y: [usize; SIZE],
}

impl<'a, const SIZE: usize> Dataset<'a, SIZE> {
    pub fn new(
        x_1: [&'a str; SIZE],
        x_2: [&'a str; SIZE],
        algorithms: [Box<dyn Algorithm>; SIZE],
        y: [usize; SIZE],
    ) -> Self {
        Self {
            x_1,
            x_2,
            algorithms,
            y,
        }
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
        use serde::Serialize;
        #[derive(Serialize)]
        struct Row<'a> {
            x_1: &'a str,
            x_2: &'a str,
            algorithm: &'a str,
            y: usize,
        }

        let mut writer = Writer::from_path(file_name)?;

        for i in 0..SIZE {
            let row = Row {
                x_1: self.x_1[i],
                x_2: self.x_2[i],
                algorithm: self.algorithms[i].name(),
                y: self.y[i],
            };

            writer.serialize(row)?;
        }

        writer.flush()?;
        Ok(())
    }

    fn export_arrow(&self, file_name: &str) -> Result<()> {
        use arrow::array::{StringArray, UInt64Array};
        use arrow::datatypes::{DataType, Field, Schema};
        use arrow::ipc::writer::FileWriter;
        use arrow::record_batch::RecordBatch;

        let x1_array = StringArray::from(self.x_1.to_vec());
        let x2_array = StringArray::from(self.x_2.to_vec());

        let alg_array =
            StringArray::from(self.algorithms.iter().map(|a| a.name()).collect::<Vec<_>>());

        let y_array = UInt64Array::from(self.y.iter().map(|&v| v as u64).collect::<Vec<_>>());

        let schema = Arc::new(Schema::new(vec![
            Field::new("x_1", DataType::Utf8, false),
            Field::new("x_2", DataType::Utf8, false),
            Field::new("algorithm", DataType::Utf8, false),
            Field::new("y", DataType::UInt64, false),
        ]));

        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(x1_array),
                Arc::new(x2_array),
                Arc::new(alg_array),
                Arc::new(y_array),
            ],
        )
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
