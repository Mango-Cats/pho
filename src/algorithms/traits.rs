use crate::algorithms::errors::AlgorithmErrors;

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
///     config_io::import,
/// };
///
/// fn main() -> Result<(), String> {
///     let config: Aline = import("tests/config_sample_aline.toml")?;
///     let _score = config.similarity("s", "s")?;
///     Ok(())
/// }
/// ```
pub trait AlgorithmTrait {
    fn similarity(&self, x: &str, y: &str) -> Result<f32, AlgorithmErrors>;
}
