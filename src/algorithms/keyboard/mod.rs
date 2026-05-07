//! keyboard
//!
//! Keyboard-proximity edit-distance similarity.
//!
//! Substitution cost between two characters is proportional to their
//! Euclidean distance on a QWERTY layout, so adjacent-key errors (a common
//! source of handwriting and typing confusions in clinical settings) are
//! penalised less than arbitrary substitutions.
//!
//! The similarity is $1 - \text{normalized\_distance}$, where the normalized
//! distance is the keyboard edit distance divided by `max(|x|, |y|)` — the
//! maximum possible cost when every position requires a full insertion or
//! deletion.

pub mod config;
mod layout;

use crate::{algorithms::Algorithm, error::Result, utils::normalize::normalize_input};
use config::Keyboard;
use layout::{MAX_KEY_DISTANCE, key_distance};

fn substitution_cost(a: char, b: char, scale: f32) -> f32 {
    if a == b {
        return 0.0;
    }
    match key_distance(a, b) {
        Some(d) => (d / MAX_KEY_DISTANCE * scale).clamp(0.0, scale),
        // Characters not in the layout (e.g. non-ASCII) → treat as full cost.
        None => scale,
    }
}

fn keyboard_edit_distance(x: &[char], y: &[char], config: &Keyboard) -> f32 {
    let m = x.len();
    let n = y.len();

    let idx = |i: usize, j: usize| i * (n + 1) + j;
    let mut dp = vec![0.0f32; (m + 1) * (n + 1)];

    for i in 1..=m {
        dp[idx(i, 0)] = dp[idx(i - 1, 0)] + config.delete_cost;
    }
    for j in 1..=n {
        dp[idx(0, j)] = dp[idx(0, j - 1)] + config.insert_cost;
    }

    for i in 1..=m {
        for j in 1..=n {
            let sub = dp[idx(i - 1, j - 1)]
                + substitution_cost(x[i - 1], y[j - 1], config.substitution_scale);
            let del = dp[idx(i - 1, j)] + config.delete_cost;
            let ins = dp[idx(i, j - 1)] + config.insert_cost;
            dp[idx(i, j)] = sub.min(del).min(ins);
        }
    }

    dp[idx(m, n)]
}

impl Algorithm for Keyboard {
    fn distance(&self, x: &str, y: &str) -> Result<f32> {
        let x_chars = normalize_input(x, self.case_insensitive);
        let y_chars = normalize_input(y, self.case_insensitive);
        Ok(keyboard_edit_distance(&x_chars, &y_chars, self))
    }

    fn normalized_distance(&self, x: &str, y: &str) -> Result<f32> {
        let x_chars = normalize_input(x, self.case_insensitive);
        let y_chars = normalize_input(y, self.case_insensitive);

        let max_len = x_chars.len().max(y_chars.len());
        if max_len == 0 {
            return Ok(0.0);
        }

        let max_cost = max_len as f32 * self.insert_cost.max(self.delete_cost);
        let raw = keyboard_edit_distance(&x_chars, &y_chars, self);
        Ok((raw / max_cost).clamp(0.0, 1.0))
    }

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

        let max_cost = m.max(n) as f32 * self.insert_cost.max(self.delete_cost);
        let raw = keyboard_edit_distance(&x_chars, &y_chars, self);
        Ok((1.0 - raw / max_cost).clamp(0.0, 1.0))
    }
}
