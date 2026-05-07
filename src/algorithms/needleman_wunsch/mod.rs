//! needleman_wunsch
//!
//! Needleman-Wunsch global sequence alignment with a drug-name-tuned
//! substitution matrix.
//!
//! The substitution matrix assigns higher scores to phonetically confusable
//! character pairs (b/p, d/t, f/v, m/n, s/z, c/k, i/y, etc.) — patterns
//! that commonly cause mix-ups in pharmaceutical prescribing.
//!
//! ## Normalization
//!
//! The raw NW score is normalized against the best possible score for the
//! pair (the higher of the two self-alignment scores):
//!
//! $$\text{similarity} = \frac{\text{NW}(x,y) - \text{worst}}{\text{best} - \text{worst}}$$
//!
//! where $\text{best} = \max(\text{NW}(x,x),\, \text{NW}(y,y))$ and
//! $\text{worst} = -\text{gap\_penalty} \times (|x| + |y|)$.

pub mod config;
pub(crate) mod matrix;
mod alignment;

use crate::{algorithms::Algorithm, error::Result, utils::normalize::normalize_input};
use alignment::nw_score;
use config::NeedlemanWunsch;
use matrix::DrugNameMatrix;

impl Algorithm for NeedlemanWunsch {
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

        let sub = DrugNameMatrix::new();
        let score = nw_score(&x_chars, &y_chars, self.gap_penalty, &sub);

        let best = sub.self_score(&x_chars).max(sub.self_score(&y_chars));
        let worst = -self.gap_penalty * (m + n) as f32;

        let range = best - worst;
        if range <= 0.0 {
            return Ok(0.0);
        }

        Ok(((score - worst) / range).clamp(0.0, 1.0))
    }
}
