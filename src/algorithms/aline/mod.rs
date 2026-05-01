//! aline
//!
//! A Rust implementation of the ALINE phonetic similarity algorithm.
//!
//! This ports the core dynamic-programming scoring logic from Kondrak (2002)
//! (and mirrors NLTK's reference implementation in `.dev/references/actual_aline.py`).
//!
//! ## What ALINE computes
//!
//! - `alignment_score(a, b)` computes the *raw* optimal **local** alignment score
//!   between two phonetic segment sequences.
//! - `similarity(a, b)` computes a normalized score in $[0, 1]$:
//!   $$\text{similarity}(a,b) = \frac{\text{alignment\_score}(a,b)}{\max(\text{alignment\_score}(a,a),\,\text{alignment\_score}(b,b))}$$
//!
//! Local alignment means the DP can restart at 0 (Smith–Waterman style), so the
//! best-matching subsequences dominate the score.
//!
//! ## Segments and Unicode
//!
//! IPA strings may contain multi-codepoint graphemes (e.g. letters with
//! combining diacritics). To avoid splitting these incorrectly, inputs are
//! tokenized into Unicode grapheme clusters.
//!
//! ## Example
//!
//! ```rust
//! use pho::{algorithms::{Aline, AlgorithmTrait}, config_io::import};
//!
//! let algo: Aline = import("tests/config_sample_aline.toml").unwrap();
//! let score = algo.similarity("s", "s").unwrap();
//! assert!((score - 1.0).abs() < 1e-6);
//! ```

mod alignment;
pub mod config;
mod scoring;
mod tokenize;

use crate::algorithms::{AlgorithmTrait, errors::AlgorithmError};
use alignment::alignment_score;
use config::Aline;
use tokenize::tokenize_and_validate;

impl AlgorithmTrait for Aline {
    fn similarity(&self, x: &str, y: &str) -> Result<f32, AlgorithmError> {
        let x_valid = tokenize_and_validate(x, self, "x")?;
        let y_valid = tokenize_and_validate(y, self, "y")?;

        let score = alignment_score(&x_valid, &y_valid, self);

        let x_self = alignment_score(&x_valid, &x_valid, self);
        let y_self = alignment_score(&y_valid, &y_valid, self);
        let denom = x_self.max(y_self);

        if denom == 0.0 {
            return Ok(0.0);
        }

        Ok(score / denom)
    }
}
