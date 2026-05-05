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
    //  Kondrak's algorithm takes in ALINE and BiSim to compute
    //  confusibility.
    let aline: Aline = import("tests/config_sample_aline.toml").unwrap();
    let bisim: BiSim = import("tests/config_sample_bisim.toml").unwrap();

    // Kondrak's algorithm
    // ...
    //  Kondrak's algorithm is a 50-50 weighted sum of Aline and BiSim
    let kondrak = EnsembleAlgorithm::new_uniform_probability(vec![
        Box::new(aline.clone()),
        Box::new(bisim.clone()),
    ])
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
