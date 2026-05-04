//! bisim
//!
//! Kondrak's BI-SIM (Bigram Similarity) algorithm.
//!
//! BI-SIM compares words by turning them into overlapping bigrams after
//! repeating the first character once, then computing the length of the
//! longest common subsequence of those bigrams. The score is normalized by
//! the longer bigram sequence length.

pub mod config;

use crate::{
    algorithms::{Algorithm, bisim::config::BiSim},
    error::Result,
    utils::normalize::normalize_input,
};

fn bigrams(chars: &[char]) -> Vec<(char, char)> {
    if chars.is_empty() {
        return Vec::new();
    }

    let mut padded = Vec::with_capacity(chars.len() + 1);
    padded.push(chars[0]);
    padded.extend(chars.iter().copied());

    padded
        .windows(2)
        .map(|window| (window[0], window[1]))
        .collect()
}

fn lcs_length(left: &[(char, char)], right: &[(char, char)]) -> usize {
    let left_len = left.len();
    let right_len = right.len();

    if left_len == 0 || right_len == 0 {
        return 0;
    }

    let mut previous = vec![0usize; right_len + 1];
    let mut current = vec![0usize; right_len + 1];

    for left_bigram in left {
        for (j, right_bigram) in right.iter().enumerate() {
            current[j + 1] = if left_bigram == right_bigram {
                previous[j] + 1
            } else {
                previous[j + 1].max(current[j])
            };
        }

        std::mem::swap(&mut previous, &mut current);
        current.fill(0);
    }

    previous[right_len]
}

impl Algorithm for BiSim {
    fn similarity(&self, x: &str, y: &str) -> Result<f32> {
        let x_chars = normalize_input(x, self.case_insensitive);
        let y_chars = normalize_input(y, self.case_insensitive);

        let x_bigrams = bigrams(&x_chars);
        let y_bigrams = bigrams(&y_chars);

        let max_len = x_bigrams.len().max(y_bigrams.len());
        if max_len == 0 {
            return Ok(1.0);
        }

        let nsim = lcs_length(&x_bigrams, &y_bigrams) as f32;
        Ok((nsim / max_len as f32).clamp(0.0, 1.0))
    }
}
