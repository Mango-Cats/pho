// src/algorithms/lcsubstring.rs
use crate::{algorithms::Algorithm, error::Result, utils::normalize::normalize_input};
use serde::{Deserialize, Serialize};

/// Longest common *substring* similarity: the length of the longest run of
/// contiguous characters shared by both inputs, normalized by the length of
/// the longer input.
///
/// Distinct from [`crate::algorithms::LCS`], which is the longest common
/// *subsequence* (characters need not be contiguous), and
/// [`crate::algorithms::LCSuf`], which only looks at a shared trailing run.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LCSubstring {
    /// Whether to perform case-insensitive comparison.
    #[serde(default)]
    pub case_insensitive: bool,
}

impl LCSubstring {
    pub fn new(case_insensitive: bool) -> Self {
        LCSubstring { case_insensitive }
    }
}

impl Algorithm for LCSubstring {
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

        let mut dp = vec![vec![0usize; n + 1]; m + 1];
        let mut longest = 0;
        for i in 1..=m {
            for j in 1..=n {
                if x_chars[i - 1] == y_chars[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    longest = longest.max(dp[i][j]);
                }
            }
        }

        let max_len = m.max(n) as f32;
        Ok(longest as f32 / max_len)
    }
}
