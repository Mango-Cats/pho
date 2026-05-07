use pho::algorithms::{Editex, Levenshtein, NGram, NGramMetric};
use pho::dataset::{Row, ScoreMatrix};
use pho::utils::io::{import, read_csv_as};

fn main() {
    // Read the big CSV
    let all: Vec<Row> = read_csv_as("D_transcribed.csv", None).unwrap();

    // Construct all algorithms
    let editex: Editex = import("tests/config_sample_editex.toml").unwrap();
    let levenshtein: Levenshtein = import("tests/config_sample_levenshtein.toml").unwrap();
    let gram3_2_0: NGram = NGram::try_new(3, 2, 0, false, NGramMetric::Dice).unwrap();

    println!("Precomputing all from the train set");
    let sm = ScoreMatrix::from_slice(
        vec![
            Box::new(editex.clone()),
            Box::new(levenshtein.clone()),
            Box::new(gram3_2_0.clone()),
        ],
        &all,
        true,
    )
    .unwrap();

    sm.export("lambert.csv").unwrap();
}
