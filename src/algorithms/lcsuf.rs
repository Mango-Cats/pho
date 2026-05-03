// src/algorithms/lcsuf.rs
use crate::{algorithms::Algorithm, error::Result};
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
