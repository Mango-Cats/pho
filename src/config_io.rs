//! Shared configuration I/O utilities.
//!
//! This module is intentionally algorithm-agnostic. It only knows how to
//! read and write TOML on disk and (de)serialize it into a caller-provided
//! type.

use serde::{Serialize, de::DeserializeOwned};
use std::fs::{self, read_to_string};

/// Load and deserialize a TOML document into any config type.
///
/// The caller owns the target schema through `T`, which keeps this parser
/// reusable across multiple algorithms.
///
/// The target schema can be:
///     - pho::algorithms::aline::config::AlineConfig
///     - pho::algorithms::editex::config::EditexConfig
pub fn import<T>(file_name: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    if !file_name.ends_with(".toml") {
        return Err("file must be a .toml".to_string());
    }

    let content = read_to_string(file_name).map_err(|e| e.to_string())?;
    toml::from_str(&content).map_err(|e| format!("TOML parse error: {e}"))
}

/// Serialize a config type into a pretty TOML document and write it to disk.
///
/// This is the inverse of [`read`]. The caller owns the schema
/// through `T`.
pub fn export<T>(file_name: &str, config: &T, append_extension: bool) -> Result<(), String>
where
    T: Serialize,
{
    let file_name = if file_name.ends_with(".toml") {
        file_name.to_string()
    } else if append_extension {
        format!("{file_name}.toml")
    } else {
        return Err("file must be a .toml".to_string());
    };

    let content =
        toml::to_string_pretty(config).map_err(|e| format!("TOML serialize error: {e}"))?;
    fs::write(file_name, content).map_err(|e| e.to_string())
}
