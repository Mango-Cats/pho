// src/learning/genetic/evolution.rs

use rand::rngs::StdRng;
use rayon::prelude::*;

use crate::ensemble::config::EnsembleConfig;
use crate::learning::loss::types::FitnessEvaluator;

use super::{
    config::GeneticConfig, crossover, mutation, population::normalize, selection::tournament_select,
};

/// Scores every individual in `population` in parallel and returns a
/// `(score, weights)` vector sorted by score ascending (best first).
/// Lower scores are considered "better" (assumes loss minimization).
pub fn score_and_rank<E: FitnessEvaluator + Sync>(
    population: Vec<Vec<f32>>,
    evaluator: &E,
) -> Vec<(f32, Vec<f32>)> {
    let mut scored: Vec<(f32, Vec<f32>)> = population
        .into_par_iter()
        .map(|w| {
            let score = evaluator.evaluate(&w);
            (score, w)
        })
        .collect();

    // Sort ascending: lower scores (losses) are better
    scored.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    scored
}

/// Produces one child from the scored population via selection, then
/// crossover, then mutation.
fn make_child(
    scored_pop: &[(f32, Vec<f32>)],
    config: &GeneticConfig,
    ensemble_config: EnsembleConfig,
    rng: &mut StdRng,
) -> Vec<f32> {
    let parent1 = tournament_select(scored_pop, config.tournament_size, rng);
    let parent2 = tournament_select(scored_pop, config.tournament_size, rng);

    let mut child = crossover::blend(parent1, parent2, rng);
    mutation::mutate(&mut child, config.mutation_rate, config.mutation_step, rng);
    normalize(&mut child, ensemble_config);
    child
}

/// Advances `population` by one generation using elitism + reproduction.
/// Returns the new population (unsorted raw individuals).
pub fn step<E: FitnessEvaluator + Sync>(
    population: Vec<Vec<f32>>,
    config: &GeneticConfig,
    ensemble_config: EnsembleConfig,
    evaluator: &E,
    rng: &mut StdRng,
) -> Vec<Vec<f32>> {
    let scored = score_and_rank(population, evaluator);

    let elite_count = config.elite_count.min(config.population_size);
    let mut next: Vec<Vec<f32>> = scored[..elite_count]
        .iter()
        .map(|(_, w)| w.clone())
        .collect();

    while next.len() < config.population_size {
        next.push(make_child(&scored, config, ensemble_config, rng));
    }

    next
}

/// Runs the full evolution loop for `config.generations` generations.
/// Returns the final population sorted best-first.
/// If `show_progress` is true, a progress bar will be displayed.
pub fn run<E: FitnessEvaluator + Sync>(
    initial_population: Vec<Vec<f32>>,
    config: &GeneticConfig,
    ensemble_config: EnsembleConfig,
    evaluator: &E,
    rng: &mut StdRng,
    show_progress: bool,
) -> Vec<(f32, Vec<f32>)> {
    use indicatif::{ProgressBar, ProgressStyle};

    let pb = if show_progress {
        let pb = ProgressBar::new(config.generations as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} generations ({eta})")
                .expect("valid template"),
        );
        pb.set_position(0);
        Some(pb)
    } else {
        None
    };

    let mut population = initial_population;

    for _ in 0..config.generations {
        population = step(population, config, ensemble_config, evaluator, rng);
        if let Some(pb) = pb.as_ref() {
            pb.inc(1);
        }
    }

    if let Some(pb) = pb.as_ref() {
        pb.finish_with_message("Evolution complete");
    }

    score_and_rank(population, evaluator)
}
