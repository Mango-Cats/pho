//! levenshtein
//!
//! A Rust implementation of the Levenshtein distance algorithm, also known
//! as edit distance.
//!
//! ## What Levenshtein computes
//!
//! - `edit_distance(a, b)` computes the minimum number of single-character
//!   edits (insertions, deletions, or substitutions) required to change
//!   string `a` into string `b`.
//! - `similarity(a, b)` computes a normalized score in $[0, 1]$:
//!   $$\text{similarity}(a,b) = 1 - \frac{\text{edit\_distance}(a,b)}{\max(|a|, |b|)}$$
//!
//! The algorithm uses dynamic programming to compute the optimal alignment.
//!
//! ## References
//!
//! - Levenshtein, V. I. (1966). "Binary codes capable of correcting deletions,
//!   insertions, and reversals". Soviet Physics Doklady.

pub mod config;
pub mod cost;

use crate::algorithms::validation::UnknownTokenError;
use config::LevenshteinConfig;

/// Compute normalized similarity between two strings using Levenshtein distance.
///
/// Returns a score in $[0, 1]$ where 1.0 means identical strings and 0.0 means
/// maximally dissimilar under the configured costs.
pub fn similarity(x: &str, y: &str, config: &LevenshteinConfig) -> Result<f32, UnknownTokenError> {
    let x_processed = if config.case_insensitive {
        x.to_lowercase()
    } else {
        x.to_string()
    };

    let y_processed = if config.case_insensitive {
        y.to_lowercase()
    } else {
        y.to_string()
    };

    let x_chars: Vec<char> = x_processed.chars().collect();
    let y_chars: Vec<char> = y_processed.chars().collect();

    let distance = edit_distance(&x_chars, &y_chars, config);
    let max_length = x_chars.len().max(y_chars.len()) as f32;

    if max_length == 0.0 {
        return Ok(1.0);
    }

    let normalized_similarity = 1.0 - (distance / max_length);
    Ok(normalized_similarity.clamp(0.0, 1.0))
}

/// Compute the Levenshtein edit distance between two character sequences.
///
/// Uses a dynamic programming table where `distance[i][j]` represents the
/// minimum cost to transform `x[0..i]` into `y[0..j]`.
fn edit_distance(x: &[char], y: &[char], config: &LevenshteinConfig) -> f32 {
    let x_length = x.len();
    let y_length = y.len();

    // Flattened (x_length + 1) x (y_length + 1) DP table
    let mut distance = vec![0.0f32; (x_length + 1) * (y_length + 1)];
    let index = |i: usize, j: usize| -> usize { i * (y_length + 1) + j };

    // Initialize first row: cost of inserting all characters of y
    for j in 1..=y_length {
        distance[index(0, j)] = distance[index(0, j - 1)] + config.costs.insert;
    }

    // Initialize first column: cost of deleting all characters of x
    for i in 1..=x_length {
        distance[index(i, 0)] = distance[index(i - 1, 0)] + config.costs.delete;
    }

    // Fill the DP table
    for i in 1..=x_length {
        for j in 1..=y_length {
            let deletion_cost = distance[index(i - 1, j)] + config.costs.delete;
            let insertion_cost = distance[index(i, j - 1)] + config.costs.insert;

            let substitution_cost = if x[i - 1] == y[j - 1] {
                distance[index(i - 1, j - 1)]
            } else {
                distance[index(i - 1, j - 1)] + config.costs.substitute
            };

            distance[index(i, j)] = deletion_cost.min(insertion_cost).min(substitution_cost);
        }
    }

    distance[index(x_length, y_length)]
}
