//! levenshtein
//!
//! A Rust implementation of the Levenshtein distance algorithm, also known
//! as edit distance.
//!
//! ## What Levenshtein computes
//!
//! - `edit_distance(a, b)` computes the minimum number of single-character
//!   edits (insertions, deletions, or substitutions) required to change
//!   string `a` into string `b`.
//! - `similarity(a, b)` computes a normalized score in $[0, 1]$:
//!   $$\text{similarity}(a,b) = 1 - \frac{\text{edit\_distance}(a,b)}{\max(|a|, |b|)}$$
//!
//! The algorithm uses dynamic programming to compute the optimal alignment.
//!
//! ## Example
//!
//! ```rust
//! use pho::{algorithms::{Levenshtein, AlgorithmTrait}, config_io::import};
//!
//! let algo: Levenshtein =
//!     import("tests/config_sample_levenshtein.toml").unwrap();
//! let score = algo.similarity("kitten", "sitting").unwrap();
//! assert!((0.0..=1.0).contains(&score));
//! ```
//!
//! ## References
//!
//! - Levenshtein, V. I. (1966). "Binary codes capable of correcting deletions,
//!   insertions, and reversals". Soviet Physics Doklady.

pub mod config;

mod distance;
mod similarity;

use crate::algorithms::AlgorithmTrait;

use config::Levenshtein;

pub(crate) use similarity::similarity;

impl AlgorithmTrait for Levenshtein {
	fn similarity(&self, x: &str, y: &str) -> Result<f32, String> {
		similarity(x, y, self).map_err(|e| e.to_string())
	}
}
