use crate::algorithms::UnknownTokenError;

use super::config::EditexConfig;
use super::distance::{edit_distance, total_delete_cost};
use super::tokenize::tokenize_and_validate;

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
