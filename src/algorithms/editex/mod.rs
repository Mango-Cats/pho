//! editex
//!
//! A Rust implementation of the Editex phonetic similarity algorithm.
//!
//! ## What Editex computes
//!
//! - `edit_distance(a, b)` computes the Editex distance between two strings.
//! - `similarity(a, b)` computes a normalized score in $[0, 1]$:
//!   $$\text{similarity}(a,b) = 1 - \frac{\text{edit\_distance}(a,b)}{\text{max\_distance}(a,b)}$$
//!
//! The edit costs are driven by phonetic groups in the config. Characters in the
//! same group are cheaper to substitute, insert, or delete than characters in
//! different groups.

use crate::algorithms::{
    editex::{
        config::EditexConfig,
        edit::{delete, replace},
    },
    validation::UnknownTokenError,
};

pub mod config;
pub mod cost;
pub mod edit;
pub mod group;

/// Compute normalized phonetic similarity between two strings.
///
/// Returns a score in $[0, 1]$ where 1.0 means identical and 0.0 means
/// maximally dissimilar under the configured Editex costs and groups.
pub fn similarity(x: &str, y: &str, config: &EditexConfig) -> Result<f32, UnknownTokenError> {
    let x_chars = tokenize_and_validate(x, config, "x")?;
    let y_chars = tokenize_and_validate(y, config, "y")?;

    let distance = edit_distance(&x_chars, &y_chars, config);
    let max_distance = total_delete_cost(&x_chars, config) + total_delete_cost(&y_chars, config);

    if max_distance == 0.0 {
        return Ok(1.0);
    }

    let similarity = 1.0 - (distance / max_distance);
    Ok(similarity.clamp(0.0, 1.0))
}

/// Convert input into lowercase ASCII chars and validate each exists in the
/// configured groups.
fn tokenize_and_validate(
    input: &str,
    config: &EditexConfig,
    input_name: &'static str,
) -> Result<Vec<char>, UnknownTokenError> {
    let chars: Vec<char> = input.chars().map(|c| c.to_ascii_lowercase()).collect();

    for (idx, symbol) in chars.iter().enumerate() {
        if !config.group.contains_key(symbol) {
            return Err(UnknownTokenError {
                token: symbol.to_string(),
                position: idx,
                input_name,
                context: "Editex config groups",
            });
        }
    }

    Ok(chars)
}

/// Editex distance using substitution/insertion/deletion costs.
fn edit_distance(x: &[char], y: &[char], config: &EditexConfig) -> f32 {
    let m = x.len();
    let n = y.len();

    let mut d = vec![0.0f32; (m + 1) * (n + 1)];
    let idx = |i: usize, j: usize| -> usize { i * (n + 1) + j };

    for i in 1..=m {
        d[idx(i, 0)] = d[idx(i - 1, 0)] + delete(x[i - 1], x.get(i - 2).copied(), config);
    }

    for j in 1..=n {
        d[idx(0, j)] = d[idx(0, j - 1)] + delete(y[j - 1], y.get(j - 2).copied(), config);
    }

    for i in 1..=m {
        for j in 1..=n {
            let delete_score = d[idx(i - 1, j)] + delete(x[i - 1], x.get(i - 2).copied(), config);
            let insert_score = d[idx(i, j - 1)] + delete(y[j - 1], y.get(j - 2).copied(), config);
            let replace_score = d[idx(i - 1, j - 1)] + replace(x[i - 1], y[j - 1], config);

            d[idx(i, j)] = delete_score.min(insert_score).min(replace_score);
        }
    }

    d[idx(m, n)]
}

fn total_delete_cost(chars: &[char], config: &EditexConfig) -> f32 {
    let mut total = 0.0;

    for (idx, symbol) in chars.iter().enumerate() {
        let previous = if idx == 0 { None } else { Some(chars[idx - 1]) };
        total += delete(*symbol, previous, config);
    }

    total
}
