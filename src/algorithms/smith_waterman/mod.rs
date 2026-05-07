//! smith_waterman
//!
//! Smith-Waterman local sequence alignment similarity.
//!
//! Unlike global alignment, Smith-Waterman finds the highest-scoring locally
//! similar region between two strings, which is useful when one drug name is a
//! prefix or contains the other (e.g. "metoprolol" vs "propranolol").
//!
//! ## Normalization
//!
//! The raw SW score is divided by `min(|x|, |y|) × match_score` — the
//! maximum possible score for the shorter string aligned perfectly —
//! giving a value in $[0, 1]$.

pub mod config;
mod alignment;

use crate::{algorithms::Algorithm, error::Result, utils::normalize::normalize_input};
use alignment::sw_score;
use config::SmithWaterman;

impl Algorithm for SmithWaterman {
    fn similarity(&self, x: &str, y: &str) -> Result<f32> {
        let x_chars = normalize_input(x, self.case_insensitive);
        let y_chars = normalize_input(y, self.case_insensitive);

        let m = x_chars.len();
        let n = y_chars.len();

        if m == 0 && n == 0 {
            return Ok(1.0);
        }
        if m == 0 || n == 0 {
            return Ok(0.0);
        }

        let max_possible = m.min(n) as f32 * self.match_score;
        if max_possible <= 0.0 {
            return Ok(0.0);
        }

        let score = sw_score(&x_chars, &y_chars, self);
        Ok((score / max_possible).clamp(0.0, 1.0))
    }
}
