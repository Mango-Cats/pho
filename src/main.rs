use pho::algorithms::{Levenshtein, Prefix};
use pho::dataset::row::split_rows;
use pho::dataset::{Row, ScoreMatrix, SplitConfig};
use pho::ensemble::config::EnsembleConfig;
use pho::ensemble::weighted_function::WeightedFunction;
use pho::learning::genetic::{GeneticConfig, optimize};
use pho::learning::loss::BinaryCrossEntropy;
use pho::{
    algorithms::{Aline, BiSim},
    ensemble::types::EnsembleAlgorithm,
    utils::io::{import, read_csv_as},
};

fn main() {
    // Read the CSV
    let rows: Vec<Row> = read_csv_as("examples/data/D_transcribed.csv", None).unwrap();

    // Get the test and train split
    let (train, test) = split_rows(
        &rows,
        &SplitConfig {
            train_fraction: 0.8,
            stratify: true,
            balance: false,
            seed: Some(67),
        },
    )
    .unwrap();

    // === # 1 Kondrak's Algorithm
    // Import the default configurations and initialize the algorithms
    // Kondrak uses
    let aline: Aline = import("tests/config_sample_aline.toml").unwrap();
    let bisim: BiSim = import("tests/config_sample_bisim.toml").unwrap();
    let ned: Levenshtein = import("tests/config_sample_levenshtein.toml").unwrap();
    let prefix: Prefix = Prefix::new(false);

    let kondrak: EnsembleAlgorithm = EnsembleAlgorithm::try_new(
        vec![
            WeightedFunction::from_similarity(aline.clone(), 0.25),
            WeightedFunction::from_similarity(bisim.clone(), 0.25),
            WeightedFunction::from_similarity(ned.clone(), 0.25),
            WeightedFunction::from_similarity(prefix.clone(), 0.25),
        ],
        EnsembleConfig::Convex,
    )
    .unwrap();

    // There is no training needed so we compute via ScoreMatrix
    // immediately
    let dataset: ScoreMatrix = ScoreMatrix::from_ensemble(&kondrak, &test, true).unwrap();
    dataset.export("base_kondrak.csv").unwrap();

    // === # 2 Kondrak's Algorithm + GA
    // Import the same algorithms from before but this time the weights
    // do not matter and set the EnsembleConfig to Linear to have
    // maximum freedom.
    let mut learned_kondrak: EnsembleAlgorithm = EnsembleAlgorithm::try_new(
        vec![
            WeightedFunction::from_similarity(aline.clone(), 0.25),
            WeightedFunction::from_similarity(bisim.clone(), 0.25),
            WeightedFunction::from_similarity(ned.clone(), 0.25),
            WeightedFunction::from_similarity(prefix.clone(), 0.25),
        ],
        EnsembleConfig::Linear,
    )
    .unwrap();

    // Precompute the ensemble given the train set
    let scores = ScoreMatrix::from_ensemble(&learned_kondrak, &train, true).unwrap();

    // Set the evaluator to BCE on the scores from the train set.
    let evaluator = BinaryCrossEntropy::new(&scores);

    // Learn better weights via genetic algorithm
    optimize(
        &mut learned_kondrak,
        &GeneticConfig {
            population_size: 200,
            generations: 100,
            mutation_rate: 0.1,
            mutation_step: 0.05,
            tournament_size: 5,
            elite_count: 3,
        },
        &evaluator,
        true,
    )
    .unwrap();

    // Now compute performance via ScoreMatrix
    let dataset: ScoreMatrix = ScoreMatrix::from_ensemble(&learned_kondrak, &test, true).unwrap();
    dataset.export("learned_kondrak.csv").unwrap();
}
