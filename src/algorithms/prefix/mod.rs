//! prefix
//!
//! Prefix similarity measures how much of the beginning of two strings
//! matches, normalized by the length of the longer input.

pub mod config;

use crate::{algorithms::Algorithm, error::Result, utils::normalize::normalize_input};

use config::Prefix;

fn common_prefix_length(x: &[char], y: &[char]) -> usize {
    x.iter()
        .zip(y.iter())
        .take_while(|(left, right)| left == right)
        .count()
}

impl Algorithm for Prefix {
    fn similarity(&self, x: &str, y: &str) -> Result<f32> {
        let x_chars = normalize_input(x, self.case_insensitive);
        let y_chars = normalize_input(y, self.case_insensitive);

        let max_len = x_chars.len().max(y_chars.len()) as f32;
        if max_len == 0.0 {
            return Ok(1.0);
        }

        let prefix_len = common_prefix_length(&x_chars, &y_chars) as f32;
        Ok((prefix_len / max_len).clamp(0.0, 1.0))
    }
}