// src/learning/genetic/population.rs

use rand::{Rng, rngs::StdRng};

/// Normalizes weights in-place so they are non-negative and sum to 1.0.
/// Any non-finite values are zeroed out before normalization.
/// If the sum is zero (all weights are zero), a uniform distribution is used instead.
// FIXME: possibly a duplicate
pub fn normalize(weights: &mut [f32]) {
    for w in weights.iter_mut() {
        if !w.is_finite() {
            *w = 0.0;
        }
        *w = w.max(0.0);
    }

    let sum: f32 = weights.iter().sum();

    if sum > 0.0 {
        weights.iter_mut().for_each(|w| *w /= sum);
    } else {
        let uniform = 1.0 / weights.len() as f32;
        weights.iter_mut().for_each(|w| *w = uniform);
    }
}

/// Creates an initial population of `population_size` individuals,
/// each with `num_weights` randomly sampled and normalized weights.
pub fn initialize(population_size: usize, num_weights: usize, rng: &mut StdRng) -> Vec<Vec<f32>> {
    (0..population_size)
        .map(|_| {
            let mut w: Vec<f32> = (0..num_weights).map(|_| rng.gen_range(0.0..1.0)).collect();
            normalize(&mut w);
            w
        })
        .collect()
}
