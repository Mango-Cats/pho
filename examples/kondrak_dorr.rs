use pho::algorithms::{Levenshtein, Prefix};
use pho::ensemble::config::EnsembleConfig;
use pho::ensemble::weighted_function::WeightedFunction;
use pho::{
    algorithms::{Aline, BiSim},
    dataset::{Row, ScoreMatrix},
    ensemble::types::EnsembleAlgorithm,
    utils::io::{import, read_csv_as},
};

fn main() {
    println!("🍜\t| # tutorial: Kondrak's algorithm");

    // Kondrak and Dorr's (2004) LASA Algorithm
    // ...
    //  This tutorial shows Kondrak and Dorr's (2004) implementation
    //  of computing confusibile drugs.

    // Load feature functions configuration
    // ...
    //  Kondrak and Dorr's algorithm takes in ALINE, BiSim, NED, and
    //  Prefix to compute confusibility.
    let aline: Aline = import("tests/config_sample_aline.toml").unwrap();
    let bisim: BiSim = import("tests/config_sample_bisim.toml").unwrap();
    let ned: Levenshtein = import("tests/config_sample_levenshtein.toml").unwrap();
    let prefix: Prefix = Prefix::new(false);

    // Kondrak and Dorr's algorithm
    // ...
    //  Kondrak and Dorr's algorithm is a average weighted sum of
    //  Aline, BiSim, NED, and Prefix.
    //
    //  Since this is an average weighted sum, we construct an
    //  ensemble that is Convex and each weight is 1/4.
    let kd = EnsembleAlgorithm::try_new(
        vec![
            WeightedFunction::from_similarity(aline.clone(), 0.25),
            WeightedFunction::from_similarity(bisim.clone(), 0.25),
            WeightedFunction::from_similarity(ned.clone(), 0.25),
            WeightedFunction::from_similarity(prefix.clone(), 0.25),
        ],
        EnsembleConfig::Convex,
    )
    .unwrap();

    // Reading a CSV
    // ...
    //  Now let's read a CSV file that contains drug name pairs,
    //  their phonetic transcriptions, and their label (0: Negative;
    //  1: Positive/LASA).
    //
    //  This requires a LARGE dataset of 400 positives, and
    //  about 160,000 negatives. Replace <yourfile> with your actual
    //  dataset.
    //
    //  Since no training is needed, we do not need to make a train-
    //  test split.
    let rows: Vec<Row> = read_csv_as("<yourfile>.csv", None).unwrap();

    // ScoreMatrix Construction
    //  ...
    //  Now we can precompute each pair from rows using Kondrak and
    //  Dorr's algorithm.
    let dataset = ScoreMatrix::from_ensemble(&kd, &rows, true).unwrap();

    dataset.export("kondrak_dorr_precomputed.csv").unwrap();
}
