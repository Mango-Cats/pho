//! Shared configuration I/O utilities.
//!
//! This module is intentionally algorithm-agnostic. It only knows how to
//! read and write TOML on disk and (de)serialize it into a caller-provided
//! type.

use crate::{Error, Result};
use serde::{Serialize, de::DeserializeOwned};
use std::fs;
use std::path::Path;
use csv;
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

/// Generic CSV reader that deserializes each row into `T` using Serde.
pub fn read_csv_as<T, P>(file_name: P) -> Result<Vec<T>>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let mut rdr = csv::Reader::from_path(file_name.as_ref())?;
    let mut out = Vec::new();

    for record in rdr.deserialize() {
        let row: T = record?;
        out.push(row);
    }

    Ok(out)
}
