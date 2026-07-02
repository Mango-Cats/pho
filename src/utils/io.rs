//! Shared configuration I/O utilities.
//!
//! This module is intentionally algorithm-agnostic. It only knows how to
//! read and write TOML on disk and (de)serialize it into a caller-provided
//! type.

use crate::{Error, Result};
use csv;
use serde::{Serialize, de::DeserializeOwned};
use std::fs;
use std::path::Path;
/// Load and deserialize a TOML document into any config type.
///
/// The caller owns the target schema through `T`, which keeps this parser
/// reusable across multiple algorithms.
///
/// The target schema can be any direct algorithm struct, for example:
///     - pho::algorithms::Aline
///     - pho::algorithms::Editex
///     - pho::algorithms::JaroWinkler
///     - pho::algorithms::Levenshtein

pub fn import<T>(file_name: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    if !file_name.ends_with(".toml") {
        return Err(Error::InvalidExtension(file_name.to_string()));
    }

    let content = fs::read_to_string(file_name)?;
    let config = toml::from_str(&content)?;

    Ok(config)
}

/// Serialize a config type into a pretty TOML document and write it to disk.
///
/// This is the inverse of [`read`]. The caller owns the schema
/// through `T`.
pub fn export<T>(file_name: &str, config: &T, append_extension: bool) -> Result<()>
where
    T: Serialize,
{
    let file_name = if file_name.ends_with(".toml") {
        file_name.to_string()
    } else if append_extension {
        format!("{file_name}.toml")
    } else {
        return Err(Error::InvalidExtension(file_name.to_string()));
    };

    let content = toml::to_string_pretty(config)?;
    fs::write(file_name, content)?;

    Ok(())
}

/// Options to control CSV parsing behaviour.
#[derive(Debug, Clone)]
pub struct CSVOptions {
    /// Field delimiter (byte). Defaults to comma.
    pub delimiter: u8,
    /// Whether the CSV file includes a header row. Defaults to `true`.
    pub has_headers: bool,
    /// Whether the CSV reader should accept records with a different number of fields.
    /// Defaults to `false`.
    pub flexible: bool,
}

impl Default for CSVOptions {
    fn default() -> Self {
        Self {
            delimiter: b',',
            has_headers: true,
            flexible: false,
        }
    }
}

/// Generic CSV reader that deserializes each row into `T` using Serde.
///
/// The caller can pass `Some(CSVOptions)` to customise parsing, or `None` to use defaults.
pub fn read_csv_as<T, P>(file_name: P, options: Option<CSVOptions>) -> Result<Vec<T>>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let opts = options.unwrap_or_default();

    let mut builder = csv::ReaderBuilder::new();
    builder
        .delimiter(opts.delimiter)
        .has_headers(opts.has_headers)
        .flexible(opts.flexible);
    let mut rdr = builder.from_path(file_name.as_ref())?;

    let mut out = Vec::new();
    for record in rdr.deserialize() {
        let row: T = record?;
        out.push(row);
    }

    Ok(out)
}

/// CSV reader that also preserves every original column for pass-through.
///
/// Like [`read_csv_as`], each record is deserialized into `T` (which only needs
/// the columns it cares about — unknown columns are ignored). In addition, this
/// returns the header names and the raw string cells of every row, so callers
/// can re-emit the untouched input columns alongside computed output.
///
/// The returned tuple is `(headers, raw_rows, items)`:
/// - `headers`: the column names. When the input has no header row, synthetic
///   names `col_0`, `col_1`, ... are generated so downstream output still has
///   labeled columns.
/// - `raw_rows`: every row as its original string cells, in input order.
/// - `items`: the deserialized `T` values, in the same order as `raw_rows`.
pub fn read_csv_records<T, P>(
    file_name: P,
    options: Option<CSVOptions>,
) -> Result<(Vec<String>, Vec<Vec<String>>, Vec<T>)>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let opts = options.unwrap_or_default();

    let mut builder = csv::ReaderBuilder::new();
    builder
        .delimiter(opts.delimiter)
        .has_headers(opts.has_headers)
        .flexible(opts.flexible);
    let mut rdr = builder.from_path(file_name.as_ref())?;

    // Clone the header record up front so we can both deserialize by name and
    // still iterate the data records afterwards.
    let header_record = if opts.has_headers {
        Some(rdr.headers()?.clone())
    } else {
        None
    };

    let mut raw_rows: Vec<Vec<String>> = Vec::new();
    let mut items = Vec::new();
    for record in rdr.records() {
        let record = record?;
        let item: T = record.deserialize(header_record.as_ref())?;
        raw_rows.push(record.iter().map(|cell| cell.to_string()).collect());
        items.push(item);
    }

    let headers = match header_record {
        Some(h) => h.iter().map(|cell| cell.to_string()).collect(),
        None => {
            let width = raw_rows.first().map(|row| row.len()).unwrap_or(0);
            (0..width).map(|i| format!("col_{i}")).collect()
        }
    };

    Ok((headers, raw_rows, items))
}
