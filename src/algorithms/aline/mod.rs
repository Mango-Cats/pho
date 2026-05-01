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
//! use pho::{algorithms::{AlineAlgorithm, AlgorithmTrait}, config_io::import};
//! use pho::algorithms::aline::config::AlineConfig;
//!
//! let config: AlineConfig = import("tests/config_sample_aline.toml").unwrap();
//! let algo = AlineAlgorithm::new(&config);
//! let score = algo.similarity("s", "s").unwrap();
//! assert!((score - 1.0).abs() < 1e-6);
//! ```

mod alignment;
pub mod config;
mod scoring;
mod tokenize;

use crate::algorithms::UnknownTokenError;
use alignment::alignment_score;
use config::AlineConfig;
use tokenize::tokenize_and_validate;

/// Compute normalized phonetic similarity between two IPA strings.
///
/// Returns a score in $[0, 1]$ where 1.0 means identical and 0.0 means
/// maximally dissimilar under the configured costs and feature weights.
pub(crate) fn similarity(x: &str, y: &str, config: &AlineConfig) -> Result<f32, UnknownTokenError> {
    let x_segments = tokenize_and_validate(x, config, "x")?;
    let y_segments = tokenize_and_validate(y, config, "y")?;

    let score = alignment_score(&x_segments, &y_segments, config);

    // Get the possible maximum similarity score to normalize `score`
    let x_self = alignment_score(&x_segments, &x_segments, config);
    let y_self = alignment_score(&y_segments, &y_segments, config);
    let denom = x_self.max(y_self);

    if denom == 0.0 {
        return Ok(0.0);
    }
    Ok(score / denom)
}
