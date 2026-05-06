// src/ensemble/similarity.rs

use super::types::EnsembleAlgorithm;
use crate::algorithms::Algorithm;
use crate::error::Result;

impl Algorithm for EnsembleAlgorithm {
    fn similarity(&self, x: &str, y: &str) -> Result<f32> {
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for entry in &self.algorithms {
            if entry.weight == 0.0 {
                continue;
            }

            let score = entry.score(x, y)?;
            weighted_sum += score * entry.weight;

            total_weight += entry.weight.abs();
        }

        if total_weight == 0.0 {
            return Ok(0.0);
        }

        Ok((weighted_sum / total_weight).clamp(0.0, 1.0))
    }
}
