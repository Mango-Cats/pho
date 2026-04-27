//! Shared configuration utilities.
//!
//! This module is intentionally algorithm-agnostic. It only knows how to
//! read TOML from disk and deserialize it into a caller-provided type.

use serde::de::DeserializeOwned;
use std::fs;

/// Load and deserialize a TOML document into any config type.
///
/// The caller owns the target schema through `T`, which keeps this parser
/// reusable across multiple algorithms.
pub fn parse_toml_file<T>(file_name: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    if !file_name.ends_with(".toml") {
        return Err("file must be a .toml".to_string());
    }

    let content = fs::read_to_string(file_name).map_err(|e| e.to_string())?;
    toml::from_str(&content).map_err(|e| format!("TOML parse error: {e}"))
}
