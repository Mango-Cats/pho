// src/learning/genetic/mutation.rs

use rand::Rng;

/// Applies random mutation to each gene in-place.
///
/// Each gene is mutated with probability `mutation_rate`.
/// When mutated, a random delta in `[-mutation_step, +mutation_step]` is added.
///
/// Weights are **not** renormalized here; callers should normalize afterward.
pub fn mutate<R: Rng>(weights: &mut [f32], mutation_rate: f32, mutation_step: f32, rng: &mut R) {
    for w in weights.iter_mut() {
        if rng.gen_range(0.0..1.0) < mutation_rate {
            *w += rng.gen_range(-mutation_step..=mutation_step);
        }
    }
}
