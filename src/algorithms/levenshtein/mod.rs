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
//! Set `consonants_only = true` in the config to strip vowels (`a`, `e`,
//! `i`, `o`, `u`; `y` stays a consonant) from both inputs before computing
//! distance — vowels are less reliably perceived than consonants, so this
//! trades exact-string sensitivity for robustness to vowel-only differences
//! (e.g. "color" vs "colour").
//!
//! ## Example
//!
//! ```rust
//! use pho::{algorithms::{Levenshtein, Algorithm}, utils::io::import};
//!
//! let algo: Levenshtein =
//!     import("algorithm_configs/eng/levenshtein.toml").unwrap();
//! let score = algo.similarity("kitten", "sitting").unwrap();
//! assert!((0.0..=1.0).contains(&score));
//! ```
//!
//! ## References
//!
//! - Levenshtein, V. I. (1966). "Binary codes capable of correcting deletions,
//!   insertions, and reversals". Soviet Physics Doklady.

pub mod config;
pub mod distance;

use crate::{algorithms::Algorithm, error::Result, utils::normalize::normalize_input};

use config::Levenshtein;
use distance::{distance, operation_counts};

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn normalized_chars(input: &str, config: &Levenshtein) -> Vec<char> {
    let chars = normalize_input(input, config.case_insensitive);
    if !config.consonants_only {
        return chars;
    }

    chars
        .into_iter()
        .filter(|c| !VOWELS.contains(&c.to_ascii_lowercase()))
        .collect()
}

impl Algorithm for Levenshtein {
    fn distance(&self, x: &str, y: &str) -> Result<f32> {
        let x_chars = normalized_chars(x, self);
        let y_chars = normalized_chars(y, self);

        Ok(distance(&x_chars, &y_chars, self))
    }

    fn normalized_distance(&self, x: &str, y: &str) -> Result<f32> {
        let x_chars = normalized_chars(x, self);
        let y_chars = normalized_chars(y, self);

        let distance = distance(&x_chars, &y_chars, self);
        let max_length = x_chars.len().max(y_chars.len()) as f32;

        if max_length == 0.0 {
            return Ok(0.0);
        }

        Ok((distance / max_length).clamp(0.0, 1.0))
    }

    fn similarity(&self, x: &str, y: &str) -> Result<f32> {
        let normalized_distance = self.normalized_distance(x, y)?;
        Ok((1.0 - normalized_distance).clamp(0.0, 1.0))
    }

    fn edit_operation_counts(&self, x: &str, y: &str) -> Result<(u32, u32, u32)> {
        let x_chars = normalized_chars(x, self);
        let y_chars = normalized_chars(y, self);

        Ok(operation_counts(&x_chars, &y_chars, self))
    }

    fn separate_enabled(&self) -> bool {
        self.separate
    }
}
