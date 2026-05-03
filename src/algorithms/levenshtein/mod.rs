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
//! use pho::{algorithms::{Levenshtein, Algorithm}, utils::io::import};
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

use crate::{
    algorithms::{Algorithm, levenshtein::distance::edit_distance},
    errors::AlgorithmError,
};

use config::Levenshtein;

impl Algorithm for Levenshtein {
    fn similarity(&self, x: &str, y: &str) -> Result<f32, AlgorithmError> {
        let x_processed = if self.case_insensitive {
            x.to_lowercase()
        } else {
            x.to_string()
        };

        let y_processed = if self.case_insensitive {
            y.to_lowercase()
        } else {
            y.to_string()
        };

        let x_chars: Vec<char> = x_processed.chars().collect();
        let y_chars: Vec<char> = y_processed.chars().collect();

        let distance = edit_distance(&x_chars, &y_chars, self);
        let max_length = x_chars.len().max(y_chars.len()) as f32;

        if max_length == 0.0 {
            return Ok(1.0);
        }

        let normalized_similarity = 1.0 - (distance / max_length);
        Ok(normalized_similarity.clamp(0.0, 1.0))
    }
}
