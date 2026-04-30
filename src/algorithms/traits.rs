/// Common interface for similarity algorithms.
///
/// Implementations should return a normalized similarity score and convert
/// algorithm-specific errors into a string message.
///
/// # Examples
///
/// ```no_run
/// use pho::{
///     algorithms::{AlineAlgorithm, SimilarityAlgorithmTrait},
///     config_io::parse_toml_file,
/// };
///
/// fn main() -> Result<(), String> {
///     let config = parse_toml_file("tests/config_sample_aline.toml")?;
///     let algo = AlineAlgorithm::new(config);
///     let _score = algo.similarity("s", "s")?;
///     Ok(())
/// }
/// ```
pub trait SimilarityAlgorithmTrait {
    fn similarity(&self, x: &str, y: &str) -> Result<f32, String>;
}
