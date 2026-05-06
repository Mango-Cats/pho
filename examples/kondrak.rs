use pho::algorithms::{Levenshtein, Prefix};
use pho::ensemble::config::EnsembleConfig;
use pho::ensemble::weighted_function::WeightedFunction;
use pho::{
    algorithms::{Aline, BiSim},
    dataset::{Dataset, Row},
    ensemble::types::EnsembleAlgorithm,
    utils::io::{import, read_csv_as},
};

fn main() {
    println!("🍜\t| # tutorial: Kondrak's algorithm");

    // Kondrak's LASA Algorithm
    // ...
    //  This tutorial shows Kondrak's implementation of computing
    //  confusibile drugs.

    // Load feature functions configuration
    // ...
    //  Kondrak's algorithm takes in ALINE, BiSim to compute
    //  confusibility.
    let aline: Aline = import("tests/config_sample_aline.toml").unwrap();
    let bisim: BiSim = import("tests/config_sample_bisim.toml").unwrap();
    let ned: Levenshtein = import("tests/config_sample_levenshtein.toml").unwrap();
    let prefix: Prefix = Prefix::new(false);

    // Kondrak's algorithm
    // ...
    //  Kondrak's algorithm is a 50-50 weighted sum of Aline and BiSim
    let kondrak = EnsembleAlgorithm::try_new(
        vec![
            WeightedFunction::from_similarity(aline.clone(), 0.25),
            WeightedFunction::from_similarity(bisim.clone(), 0.25),
            WeightedFunction::from_normalized_distance(ned.clone(), 0.25),
            WeightedFunction::from_similarity(prefix.clone(), 0.25),
        ],
        EnsembleConfig::Convex,
    )
    .unwrap();

    // Reading a CSV
    // ...
    //  Now let's read a CSV file that contains drug name pairs,
    //  their phonetic transcriptions, and their label (0: Unlabeled;
    //  1: Positive/LASA)
    let rows: Vec<Row> = read_csv_as("examples/data/sample_lasa.csv", None).unwrap();

    // Example row
    println!("\t| {:?}", rows[0]);

    // Dataset Construction
    //  ...
    //  Now let's construct a dataset and compute the score of each
    //  drug pair on Kondrak's algorithm. Then export it.
    //
    //  Looking through the resulting database, the first half must
    //  be all LASA drugs with the ensemble score being high.
    //  While the lower half contains unlabeled drugs so most of them
    //  (if not all) should have a low ensemble score.
    let dataset = Dataset::from_ensemble(&kondrak, &rows).unwrap();

    dataset.export("example_dataset_aline.csv").unwrap();
}
