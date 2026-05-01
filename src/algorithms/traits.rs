/// Common interface for similarity algorithms.
///
/// Implementations should return a normalized similarity score and convert
/// algorithm-specific errors into a string message.
///
/// # Examples
///
/// ```no_run
/// use pho::{
///     algorithms::{AlineAlgorithm, AlgorithmTrait},
///     config_io::read,
/// };
///
/// fn main() -> Result<(), String> {
///     let config = read("tests/config_sample_aline.toml")?;
///     let algo = AlineAlgorithm::new(&config);
///     let _score = algo.similarity("s", "s")?;
///     Ok(())
/// }
/// ```
pub trait AlgorithmTrait {
    fn similarity(&self, x: &str, y: &str) -> Result<f32, String>;
}
