// src/algorithms/lcs.rs
use crate::{algorithms::Algorithm, error::Result, utils::normalize::normalize_input};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LCS {
    /// Whether to perform case-insensitive comparison.
    pub case_insensitive: bool,
}

impl LCS {
    pub fn new(case_insensitive: bool) -> Self {
        return LCS { case_insensitive };
    }
}

impl Algorithm for LCS {
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

        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                if x_chars[i - 1] == y_chars[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }

        let lcs_len = dp[m][n] as f32;
        let max_len = m.max(n) as f32;

        Ok(lcs_len / max_len)
    }
}
