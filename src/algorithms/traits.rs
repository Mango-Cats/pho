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
///     let config: Aline = import("algorithm_configs/eng/aline.toml").unwrap();
///     let _score = config.similarity("s", "s").unwrap();
/// }
/// ```
pub trait Algorithm: Send + Sync {
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

    /// Returns the number of substitutions, insertions, and deletions used
    /// by the minimal-cost alignment between two inputs, as `(substitutions,
    /// insertions, deletions)`.
    ///
    /// Unlike [`Algorithm::distance`], these are literal operation tallies
    /// from the alignment path, not weighted costs — so they do not
    /// necessarily sum back to `distance(x, y)` for algorithms with
    /// non-uniform costs.
    ///
    /// Algorithms without a well-defined edit-operation decomposition can
    /// keep the default implementation, which reports that separated counts
    /// are unsupported.
    fn edit_operation_counts(&self, x: &str, y: &str) -> Result<(u32, u32, u32)> {
        let _ = (x, y);
        Err(crate::Error::SeparatedCountsNotSupported {
            algorithm: self.name(),
        })
    }

    /// Whether this algorithm's config requests separated edit-operation
    /// counts (via a `separate = true` key) instead of a single summed
    /// distance/similarity column.
    fn separate_enabled(&self) -> bool {
        false
    }

    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
            .rsplit("::")
            .next()
            .unwrap_or("Algorithm")
    }
}
