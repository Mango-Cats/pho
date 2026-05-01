//! jaro_winkler
//!
//! A Rust implementation of the Jaro-Winkler similarity algorithm.
//!
//! ## What Jaro-Winkler computes
//!
//! The Jaro-Winkler similarity is a string metric measuring edit distance
//! between two sequences. It is a variant of the Jaro distance metric that
//! gives more favorable ratings to strings with common prefixes.
//!
//! - `jaro_similarity(a, b)` computes the base Jaro similarity in $[0, 1]$
//! - `similarity(a, b)` computes the Jaro-Winkler similarity which adds a
//!   prefix bonus to the Jaro score:
//!   $$\text{jaro\_winkler} = \text{jaro} + (L \times P \times (1 - \text{jaro}))$$
//!   where $L$ is the length of common prefix (up to max 4 characters) and
//!   $P$ is the prefix scaling factor (typically 0.1).
//!
//! ## Use cases
//!
//! Jaro-Winkler is particularly effective for:
//! - Short strings (e.g., names)
//! - Strings where typos are more likely at the end than the beginning
//! - Record linkage and deduplication tasks
//!
//! ## Example
//!
//! ```rust
//! use pho::{algorithms::{JaroWinkler, AlgorithmTrait}, config_io::import};
//!
//! let algo: JaroWinkler =
//!     import("tests/config_sample_jaro_winkler.toml").unwrap();
//! let score = algo.similarity("dixon", "dixon").unwrap();
//! assert!((score - 1.0).abs() < 1e-6);
//! ```
//!
//! ## References
//!
//! - Jaro, M. A. (1989). "Advances in record linkage methodology"
//! - Winkler, W. E. (1990). "String Comparator Metrics and Enhanced Decision Rules"

pub mod config;

mod jaro;
mod winkler;

use crate::algorithms::AlgorithmTrait;

use config::JaroWinkler;

pub(crate) use winkler::similarity;

impl AlgorithmTrait for JaroWinkler {
    fn similarity(&self, x: &str, y: &str) -> Result<f32, String> {
        similarity(x, y, self).map_err(|e| e.to_string())
    }
}
