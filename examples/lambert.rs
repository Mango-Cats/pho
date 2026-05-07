use pho::algorithms::{Editex, Levenshtein, NGram, NGramMetric};
use pho::dataset::{Row, ScoreMatrix};
use pho::utils::io::{import, read_csv_as};

fn main() {
    // Lambert's (1999) LASA Algorithm
    // ...
    //  This tutorial shows Lambert's (1999) implementation of
    //  computing confusibile drugs.

    // Load feature functions configuration
    // ...
    //  Lambert's algorithm takes in Editex, Levenshtein, and
    //  Trigram-2B to compute confusibility.
    //
    //  Since we will learn the weight of each feature function via
    //  Logistic Regression we will not create an ensemble here
    //  nor will we initialize any weights.
    let editex: Editex = import("tests/config_sample_editex.toml").unwrap();
    let levenshtein: Levenshtein = import("tests/config_sample_levenshtein.toml").unwrap();
    let gram3_2_0: NGram = NGram::try_new(3, 2, 0, false, NGramMetric::Dice).unwrap();

    // Reading a CSV
    // ...
    //  Now let's read a CSV file that contains drug name pairs,
    //  their phonetic transcriptions, and their label (0: Negative;
    //  1: Positive/LASA).
    //
    //  This requires a dataset of equal numbers of Positive and
    //  Negative pairs. Replace <yourfile> with your actual
    //  dataset.
    //
    //  Since this will export a dataset to be used as training you
    //  can opt to use `split_rows()` to do it for you.
    let all: Vec<Row> = read_csv_as("<yourfile>.csv", None).unwrap();

    // ScoreMatrix Construction
    //  ...
    //  Now we can precompute each pair from rows using each of the
    //  feature functions used in Lambert. This ScoreMatrix
    //  essentially constructs the tabular data that will be used to
    //  train the Logistic Regression algorithm.
    let lambert_data = ScoreMatrix::from_slice(
        vec![
            Box::new(editex.clone()),
            Box::new(levenshtein.clone()),
            Box::new(gram3_2_0.clone()),
        ],
        &all,
        true,
    )
    .unwrap();

    lambert_data.export("lambert.csv").unwrap();
}
