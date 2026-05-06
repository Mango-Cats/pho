use pho::ensemble::weighted_function::WeightedFunction;
use pho::{
    self,
    algorithms::{BiSim, JaroWinkler, Levenshtein, NGram},
    dataset::{Dataset, Row},
    ensemble::{config::EnsembleConfig, types::EnsembleAlgorithm},
    utils::io::{import, read_csv_as},
};

fn main() {
    println!("🍜\t| # tutorial: dataset");
    // Datasets
    // ...
    //  Later on we will work with learning the weights of an ensemble
    //  algorithm. But before we can do this, we must first look into
    //  how datasets are constructed and used.

    // Load and validate configs
    let levenshtein = import::<Levenshtein>("tests/config_sample_levenshtein.toml").unwrap();
    let jaro_winkler = import::<JaroWinkler>("tests/config_sample_jaro_winkler.toml").unwrap();
    let ngram_2_1_1_dice = import::<NGram>("tests/config_sample_ngram.toml").unwrap();
    let bisim = import::<BiSim>("tests/config_sample_bisim.toml").unwrap();
    jaro_winkler.validate().unwrap();
    ngram_2_1_1_dice.validate().unwrap();

    // Make an ensemble
    let ensemble = EnsembleAlgorithm::try_new(
        vec![
            WeightedFunction::from_similarity(levenshtein.clone(), 1.0),
            WeightedFunction::from_similarity(jaro_winkler.clone(), 1.0),
            WeightedFunction::from_similarity(ngram_2_1_1_dice.clone(), 1.0),
            WeightedFunction::from_similarity(bisim.clone(), 1.0),
        ],
        EnsembleConfig::Linear,
    )
    .unwrap();

    // Loading CSVs
    // ...
    //  Most data comes from CSV files. The `read_csv_as` function
    //  allows us to load CSVs and store it as a Vec<T>. Here, `T`
    //  means the data types of the values stored in each column of
    //  the CSV.
    //
    //  So, in this example, since we're reading this CSV as
    //  T: Row
    //  Our resulting `rows` will have the data type of
    //  Vec<Row>
    //
    //  `Row` supports common CSV headers via serde aliases,
    //  so this stays concise while still being explicit about shape.
    let rows = read_csv_as::<Row, _>("examples/data/dataset.csv", None).unwrap();

    // Building the Dataset
    // ...
    //  There are two ways to build a dataset:
    //      1. `from_slice`, we build a dataset form a slice of
    //      (unweighted) Algorithms.
    //
    //      2. `from_ensemble`, we build a dataset from an ensemble.
    //
    //  For this example, we will build both to see the difference.

    // Dataset from a slice of algorithms
    let slice_dataset = Dataset::from_slice(
        vec![
            Box::new(levenshtein.clone()),
            Box::new(jaro_winkler.clone()),
            Box::new(ngram_2_1_1_dice.clone()),
            Box::new(bisim.clone()),
        ],
        &rows,
        true,
    )
    .unwrap();

    // Dataset from an ensemble
    let slice_ensemble = Dataset::from_ensemble(&ensemble, &rows, true).unwrap();

    // Save both datasets to compare results
    slice_dataset
        .export("example_dataset_slice.csv.csv")
        .unwrap();
    slice_ensemble
        .export("example_dataset_ensemble.csv")
        .unwrap()
}
