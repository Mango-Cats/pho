// src/algorithms/lcsuf.rs
use crate::{algorithms::Algorithm, error::Result, utils::normalize::normalize_input};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LCSuf {
    /// Whether to perform case-insensitive comparison.
    pub case_insensitive: bool,
}

impl LCSuf {
    pub fn new(case_insensitive: bool) -> Self {
        LCSuf { case_insensitive }
    }
}

impl Algorithm for LCSuf {
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

        let mut match_count = 0;
        for (cx, cy) in x_chars.iter().rev().zip(y_chars.iter().rev()) {
            if cx == cy {
                match_count += 1;
            } else {
                break;
            }
        }

        let max_len = m.max(n) as f32;
        Ok(match_count as f32 / max_len)
    }
}
