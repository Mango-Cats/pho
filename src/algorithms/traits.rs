use crate::errors::AlgorithmError;

/// Common interface for similarity algorithms.
///
/// Implementations should return a normalized similarity score and convert
/// algorithm-specific errors into a string message.
///
/// # Examples
///
/// ```no_run
/// use pho::{
///     algorithms::{Aline, AlgorithmTrait},
///     utils::io::import,
/// };
///
/// fn main() {
///     let config: Aline = import("tests/config_sample_aline.toml").unwrap();
///     let _score = config.similarity("s", "s").unwrap();
/// }
/// ```
pub trait AlgorithmTrait {
    fn similarity(&self, x: &str, y: &str) -> Result<f32, AlgorithmError>;
}
