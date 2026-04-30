//! # Algorithms
//!

pub mod aline;
pub mod editex;
pub mod jaro_winkler;
pub mod levenshtein;

mod dispatcher;
mod errors;
mod registry;
mod traits;
mod wrappers;

pub use dispatcher::SimilarityAlgorithm;
pub use errors::UnknownTokenError;
pub use registry::AlgorithmConfig;
pub use traits::SimilarityAlgorithmTrait;
pub use wrappers::{AlineAlgorithm, EditexAlgorithm, JaroWinklerAlgorithm, LevenshteinAlgorithm};
