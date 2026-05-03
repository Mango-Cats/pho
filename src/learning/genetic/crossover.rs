// src/learning/genetic/crossover.rs

use rand::Rng;

/// Performs blend (BLX-a style) crossover between two parents.
///
/// Each child gene is a convex combination: `a * p1 + (1 − α) * p2`
/// where `a` is sampled uniformly from [0, 1) for the entire individual.
///
/// The result is **not** normalized here; callers should normalize afterward
/// if needed (e.g. after mutation).
pub fn blend<R: Rng>(parent1: &[f32], parent2: &[f32], rng: &mut R) -> Vec<f32> {
    let alpha: f32 = rng.gen_range(0.0..1.0);
    parent1
        .iter()
        .zip(parent2.iter())
        .map(|(w1, w2)| alpha * w1 + (1.0 - alpha) * w2)
        .collect()
}
