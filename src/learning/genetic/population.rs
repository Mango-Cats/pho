// src/learning/genetic/population.rs

use crate::ensemble::config::EnsembleConfig;
use rand::{Rng, rngs::StdRng};

/// Normalizes weights in-place according to the ensemble config constraints.
/// Any non-finite values are zeroed out before normalization.
///
/// - **Linear**: No normalization applied (weights can be any finite value)
/// - **Conical**: Weights made non-negative (no sum requirement)
/// - **Affine**: Weights scaled to sum to 1.0 (can be negative)
/// - **Convex**: Weights made non-negative and scaled to sum to 1.0
pub fn normalize(weights: &mut [f32], config: EnsembleConfig) {
    // First, handle non-finite values
    for w in weights.iter_mut() {
        if !w.is_finite() {
            *w = 0.0;
        }
    }

    match config {
        EnsembleConfig::Linear => {
            // No constraints; weights can be any finite value
            // Nothing to do after cleaning up non-finite values
        }
        EnsembleConfig::Conical => {
            // Weights must be >= 0.0 (no sum requirement)
            for w in weights.iter_mut() {
                *w = w.max(0.0);
            }
        }
        EnsembleConfig::Affine => {
            // Weights must sum to 1.0 (can be negative)
            let sum: f32 = weights.iter().sum();
            // Use same threshold as Convex for consistency
            if sum > 1e-6 || sum < -1e-6 {
                weights.iter_mut().for_each(|w| *w /= sum);
            } else {
                let uniform = 1.0 / weights.len() as f32;
                weights.iter_mut().for_each(|w| *w = uniform);
            }
        }
        EnsembleConfig::Convex => {
            // Weights must be non-negative and sum to 1.0
            for w in weights.iter_mut() {
                *w = w.max(0.0);
            }
            let sum: f32 = weights.iter().sum();
            // Use small epsilon threshold to avoid numerical issues with very small sums
            if sum > 1e-6 {
                weights.iter_mut().for_each(|w| *w /= sum);
            } else {
                let uniform = 1.0 / weights.len() as f32;
                weights.iter_mut().for_each(|w| *w = uniform);
            }
        }
    }
}

/// Creates an initial population of `population_size` individuals,
/// each with `num_weights` randomly sampled and normalized weights.
pub fn initialize(
    population_size: usize,
    num_weights: usize,
    config: EnsembleConfig,
    rng: &mut StdRng,
) -> Vec<Vec<f32>> {
    (0..population_size)
        .map(|_| {
            let mut w: Vec<f32> = (0..num_weights).map(|_| rng.gen_range(0.0..1.0)).collect();
            normalize(&mut w, config);
            w
        })
        .collect()
}
