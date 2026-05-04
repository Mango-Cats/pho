// src/error.rs

use thiserror::Error;

/// The unified error type for the entire `pho` crate.
#[derive(Error, Debug)]
pub enum Error {
    #[error(
        "Unknown token '{token}' at position {position} in {input_name} (not found in {context})"
    )]
    UnknownToken {
        token: String,
        position: usize,
        input_name: &'static str,
        context: &'static str,
    },

    #[error("{feature} values must sum to 1.0, but got {sum}")]
    InvalidFeatureSum { feature: &'static str, sum: f32 },

    #[error("Epsilon must be non-negative, got {0}")]
    NegativeEpsilon(f32),

    #[error("Jaro-Winkler prefix_scale must be in [0.0, 0.25], got {0}")]
    InvalidPrefixScale(f32),

    #[error("Ensemble must contain at least one algorithm")]
    EmptyEnsemble,

    #[error("Ensemble weight must be finite, got {0}")]
    NonFiniteWeight(f32),

    #[error("Ensemble weight must be non-negative, got {0}")]
    NegativeWeight(f32),

    #[error("Ensemble weights must sum to 1.0, got {0}")]
    WeightsDoNotSumToOne(f32),

    #[error("Ensemble weight must be valid (finite and non-negative), got {0}")]
    InvalidWeight(f32),

    #[error("File must have a .toml extension: {0}")]
    InvalidExtension(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Invalid dataset shape: {0}")]
    InvalidDatasetShape(String),
}

/// A convenient alias for Result types within this crate.
pub type Result<T> = std::result::Result<T, Error>;
