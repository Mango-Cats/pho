use crate::algorithms::UnknownTokenError;

use super::config::JaroWinkler;
use super::jaro::jaro_similarity;

/// Compute Jaro-Winkler similarity between two strings.
///
/// Returns a score in $[0, 1]$ where 1.0 means identical strings and 0.0
/// means no similarity. The Jaro-Winkler metric applies a prefix bonus to
/// the base Jaro similarity.
pub(crate) fn similarity(x: &str, y: &str, config: &JaroWinkler) -> Result<f32, UnknownTokenError> {
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
