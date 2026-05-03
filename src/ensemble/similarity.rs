use crate::errors::AlgorithmError;

use super::types::{EnsembleAlgorithm, WeightedAlgorithm};

fn weighted_score(
    entry: &WeightedAlgorithm,
    x: &str,
    y: &str,
) -> Result<Option<(f32, f32)>, AlgorithmError> {
    if !entry.weight.is_finite() {
        return Err(AlgorithmError::Special(
            "EnsembleAlgorithm weight must be finite".to_string(),
        ));
    }
    if entry.weight < 0.0 {
        return Err(AlgorithmError::Special(
            "EnsembleAlgorithm weight must be non-negative".to_string(),
        ));
    }
    if entry.weight == 0.0 {
        return Ok(None);
    }

    let score = entry.algorithm.similarity(x, y)?;
    Ok(Some((score, entry.weight)))
}

/// Compute weighted similarity using an ensemble configuration.
pub fn similarity(x: &str, y: &str, ensemble: &EnsembleAlgorithm) -> Result<f32, AlgorithmError> {
    if ensemble.algorithms.is_empty() {
        return Err(AlgorithmError::Special(
            "EnsembleAlgorithm requires at least one algorithm".to_string(),
        ));
    }

    let mut weighted_sum = 0.0;
    let mut total_weight = 0.0;

    for entry in &ensemble.algorithms {
        if let Some((score, weight)) = weighted_score(entry, x, y)? {
            weighted_sum += score * weight;
            total_weight += weight;
        }
    }

    if total_weight == 0.0 {
        return Err(AlgorithmError::Special(
            "EnsembleAlgorithm requires at least one positive weight".to_string(),
        ));
    }

    Ok((weighted_sum / total_weight).clamp(0.0, 1.0))
}
