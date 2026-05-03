// src/learning/genetic/selection.rs

use rand::{Rng, seq::SliceRandom};

/// Selects one individual via tournament selection.
///
/// Randomly samples `tournament_size` candidates from the scored population
/// (which must be non-empty) and returns the one with the highest fitness score.
pub fn tournament_select<'a, R: Rng>(
    scored_pop: &'a [(f32, Vec<f32>)],
    tournament_size: usize,
    rng: &mut R,
) -> &'a [f32] {
    let mut best = scored_pop
        .choose(rng)
        .expect("population must not be empty");

    for _ in 1..tournament_size {
        let candidate = scored_pop.choose(rng).unwrap();
        if candidate.0 > best.0 {
            best = candidate;
        }
    }

    &best.1
}
