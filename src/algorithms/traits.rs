use crate::error::Result;

/// Common interface for similarity algorithms.
///
/// Implementations should return a normalized similarity score and convert
/// algorithm-specific errors into a string message.
///
/// # Examples
///
/// ```no_run
/// use pho::{
///     algorithms::{Aline, Algorithm},
///     utils::io::import,
/// };
///
/// fn main() {
///     let config: Aline = import("tests/config_sample_aline.toml").unwrap();
///     let _score = config.similarity("s", "s").unwrap();
/// }
/// ```
pub trait Algorithm {
    fn similarity(&self, x: &str, y: &str) -> Result<f32>;

    /// Returns the raw distance between two inputs when the algorithm
    /// defines one.
    ///
    /// Similarity-only algorithms can keep the default implementation,
    /// which reports that distance is unsupported.
    fn distance(&self, x: &str, y: &str) -> Result<f32> {
        let _ = (x, y);
        Err(crate::Error::DistanceNotSupported {
            algorithm: self.name(),
        })
    }

    /// Returns the normalized distance between two inputs when the
    /// algorithm defines one.
    ///
    /// This is useful for algorithms with a native distance scale and a
    /// predictable normalization range.
    fn normalized_distance(&self, x: &str, y: &str) -> Result<f32> {
        let _ = (x, y);
        Err(crate::Error::DistanceNotSupported {
            algorithm: self.name(),
        })
    }

    /// Whether this algorithm requires phonetic transcriptions instead of
    /// raw orthographic forms when constructing learning datasets.
    fn requires_transcription(&self) -> bool {
        false
    }

    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
            .rsplit("::")
            .next()
            .unwrap_or("Algorithm")
    }
}
