//! jaro_winkler
//!
//! A Rust implementation of the Jaro-Winkler similarity algorithm.
//!
//! ## What Jaro-Winkler computes
//!
//! The Jaro-Winkler similarity is a string metric measuring edit distance
//! between two sequences. It is a variant of the Jaro distance metric that
//! gives more favorable ratings to strings with common prefixes.
//!
//! - `jaro_similarity(a, b)` computes the base Jaro similarity in $[0, 1]$
//! - `similarity(a, b)` computes the Jaro-Winkler similarity which adds a
//!   prefix bonus to the Jaro score:
//!   $$\text{jaro\_winkler} = \text{jaro} + (L \times P \times (1 - \text{jaro}))$$
//!   where $L$ is the length of common prefix (up to max 4 characters) and
//!   $P$ is the prefix scaling factor (typically 0.1).
//!
//! ## Use cases
//!
//! Jaro-Winkler is particularly effective for:
//! - Short strings (e.g., names)
//! - Strings where typos are more likely at the end than the beginning
//! - Record linkage and deduplication tasks
//!
//! ## References
//!
//! - Jaro, M. A. (1989). "Advances in record linkage methodology"
//! - Winkler, W. E. (1990). "String Comparator Metrics and Enhanced Decision Rules"

use crate::algorithms::validation::UnknownTokenError;

pub mod config;

use config::JaroWinklerConfig;

/// Compute Jaro-Winkler similarity between two strings.
///
/// Returns a score in $[0, 1]$ where 1.0 means identical strings and 0.0
/// means no similarity. The Jaro-Winkler metric applies a prefix bonus to
/// the base Jaro similarity.
pub fn similarity(x: &str, y: &str, config: &JaroWinklerConfig) -> Result<f32, UnknownTokenError> {
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

    let jaro_score = jaro_similarity(&x_chars, &y_chars);

    if jaro_score == 0.0 {
        return Ok(0.0);
    }

    // Compute common prefix length
    let prefix_length = common_prefix_length(&x_chars, &y_chars, config.max_prefix_length);

    // Apply Winkler modification
    let jaro_winkler_score =
        jaro_score + (prefix_length as f32 * config.prefix_scale * (1.0 - jaro_score));

    Ok(jaro_winkler_score.clamp(0.0, 1.0))
}

/// Compute the base Jaro similarity between two character sequences.
///
/// The Jaro similarity considers:
/// - Matching characters (characters that are the same and within a certain distance)
/// - Transpositions (matching characters that are out of order)
fn jaro_similarity(x: &[char], y: &[char]) -> f32 {
    let x_length = x.len();
    let y_length = y.len();

    if x_length == 0 && y_length == 0 {
        return 1.0;
    }

    if x_length == 0 || y_length == 0 {
        return 0.0;
    }

    // Maximum allowed distance for matching characters
    let match_distance = (x_length.max(y_length) / 2).saturating_sub(1);

    let mut x_matches = vec![false; x_length];
    let mut y_matches = vec![false; y_length];

    let mut matching_characters = 0;
    let mut transpositions = 0;

    // Find matching characters
    for i in 0..x_length {
        let start = i.saturating_sub(match_distance);
        let end = (i + match_distance + 1).min(y_length);

        for j in start..end {
            if y_matches[j] || x[i] != y[j] {
                continue;
            }

            x_matches[i] = true;
            y_matches[j] = true;
            matching_characters += 1;
            break;
        }
    }

    if matching_characters == 0 {
        return 0.0;
    }

    // Count transpositions
    let mut y_position = 0;
    for i in 0..x_length {
        if !x_matches[i] {
            continue;
        }

        while !y_matches[y_position] {
            y_position += 1;
        }

        if x[i] != y[y_position] {
            transpositions += 1;
        }

        y_position += 1;
    }

    let matching_characters_f32 = matching_characters as f32;
    let transpositions_f32 = (transpositions / 2) as f32;

    // Jaro similarity formula
    (matching_characters_f32 / x_length as f32
        + matching_characters_f32 / y_length as f32
        + (matching_characters_f32 - transpositions_f32) / matching_characters_f32)
        / 3.0
}

/// Compute the length of the common prefix between two strings.
///
/// Returns the number of matching characters at the beginning of both
/// strings, up to `max_length`.
fn common_prefix_length(x: &[char], y: &[char], max_length: usize) -> usize {
    let mut prefix_length = 0;
    let limit = x.len().min(y.len()).min(max_length);

    for i in 0..limit {
        if x[i] == y[i] {
            prefix_length += 1;
        } else {
            break;
        }
    }

    prefix_length
}
