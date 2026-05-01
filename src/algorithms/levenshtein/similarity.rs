use crate::algorithms::UnknownTokenError;

use super::config::LevenshteinConfig;
use super::distance::edit_distance;

/// Compute normalized similarity between two strings using Levenshtein distance.
///
/// Returns a score in $[0, 1]$ where 1.0 means identical strings and 0.0 means
/// maximally dissimilar under the configured costs.
pub(crate) fn similarity(
    x: &str,
    y: &str,
    config: &LevenshteinConfig,
) -> Result<f32, UnknownTokenError> {
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
