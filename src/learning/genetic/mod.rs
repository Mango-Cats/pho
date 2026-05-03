// src/learning/genetic/mod.rs

pub mod config;
pub mod crossover;
pub mod evolution;
pub mod mutation;
pub mod population;
pub mod selection;

use rand::{SeedableRng, rngs::StdRng};

use crate::ensemble::types::EnsembleAlgorithm;
use crate::error::{Error, Result};
use crate::learning::loss::types::FitnessEvaluator;

pub use config::GeneticConfig;

/// Optimizes the weights of an [`EnsembleAlgorithm`] using a genetic algorithm.
///
/// # Errors
/// Returns an error if the ensemble is empty, `population_size` is zero,
/// or the best weights found are invalid (non-finite or negative).
pub fn optimize<E: FitnessEvaluator>(
    ensemble: &mut EnsembleAlgorithm,
    config: &GeneticConfig,
    evaluator: &E,
) -> Result<()> {
    validate_inputs(ensemble, config)?;

    let num_weights = ensemble.algorithms.len();
    let mut rng = StdRng::from_entropy();

    let initial = population::initialize(config.population_size, num_weights, &mut rng);
    let final_ranked = evolution::run(initial, config, evaluator, &mut rng);

    apply_best_weights(ensemble, &final_ranked[0].1)?;
    ensemble.validate()?;

    Ok(())
}

fn validate_inputs(ensemble: &EnsembleAlgorithm, config: &GeneticConfig) -> Result<()> {
    if ensemble.algorithms.is_empty() {
        return Err(Error::EmptyEnsemble);
    }
    if config.population_size == 0 {
        return Err(Error::InvalidFeatureSum {
            feature: "population_size",
            sum: 0.0,
        });
    }
    Ok(())
}

fn apply_best_weights(ensemble: &mut EnsembleAlgorithm, weights: &[f32]) -> Result<()> {
    for (entry, &w) in ensemble.algorithms.iter_mut().zip(weights.iter()) {
        if !w.is_finite() {
            return Err(Error::NonFiniteWeight(w));
        }
        if w < 0.0 {
            return Err(Error::NegativeWeight(w));
        }
        entry.weight = w;
    }
    Ok(())
}
