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
use crate::learning::loss::FitnessEvaluator;

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
            return Err(Error::NegativeWeight);
        }
        entry.weight = w;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::Algorithm;
    use crate::algorithms::{LCS, LCSuf};
    use crate::dataset::{Dataset, Row};
    use crate::ensemble::config::EnsembleConfig;
    use crate::ensemble::weighted_function::WeightedFunction;
    use crate::learning::loss::mse::MeanSquaredError;

    #[test]
    fn test_ga_synthetic_prefix_dominance() {
        let ground_truth = vec![
            ("bananamonster", "bananacoolguy", 0.45),
            ("bananamrsunshine", "bananalucioooo", 0.40),
            ("bananaphone", "bananabread", 0.55),
            ("bananapancakes", "bananasplit", 0.45),
        ];

        let lcs = LCS {
            case_insensitive: true,
        };
        let lcsuf = LCSuf {
            case_insensitive: true,
        };

        let mut ensemble = EnsembleAlgorithm {
            algorithms: vec![
                WeightedFunction::from_function(
                    lcs.name(),
                    0.1,
                    lcs.requires_transcription(),
                    move |x, y| lcs.similarity(x, y),
                ),
                WeightedFunction::from_function(
                    lcsuf.name(),
                    0.9,
                    lcsuf.requires_transcription(),
                    move |x, y| lcsuf.similarity(x, y),
                ),
            ],
            mode: EnsembleConfig::Convex,
        };

        let rows = ground_truth
            .iter()
            .map(|(x, y, label)| Row::builder(*x, *y).label(*label).build())
            .collect::<Vec<_>>();
        let training_data = Dataset::from_ensemble(&ensemble, &rows).unwrap();
        let evaluator = MeanSquaredError::new(&training_data);

        let config = GeneticConfig {
            population_size: 100,
            generations: 50,
            mutation_rate: 0.2,
            mutation_step: 0.1,
            tournament_size: 5,
            elite_count: 2,
        };

        optimize(&mut ensemble, &config, &evaluator).unwrap();

        let final_lcs_weight = ensemble.algorithms[0].weight;
        let final_lcsuf_weight = ensemble.algorithms[1].weight;

        println!("Final LCS Weight: {:.4}", final_lcs_weight);
        println!("Final LCSuf Weight: {:.4}", final_lcsuf_weight);

        assert!(
            final_lcs_weight > final_lcsuf_weight,
            "Genetic algorithm failed to identify the dominant feature! LCS: {}, LCSuf: {}",
            final_lcs_weight,
            final_lcsuf_weight
        );

        assert!(final_lcs_weight > 0.8);
    }
}
