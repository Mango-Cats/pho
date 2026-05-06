use pho::algorithms::{Editex, JaroWinkler, Levenshtein, NGram, NGramMetric, Prefix};
use pho::dataset::row::split_rows;
use pho::dataset::{Row, ScoreMatrix, SplitConfig};
use pho::{
    algorithms::{Aline, BiSim},
    utils::io::{import, read_csv_as},
};

fn main() {
    // Read the big CSV
    let rows: Vec<Row> = read_csv_as("D_transcribed.csv", None).unwrap();

    // Get the test and train split
    let (train, _test) = split_rows(
        &rows,
        &SplitConfig {
            train_fraction: 0.8,
            stratify: true,
            balance: false,
            seed: Some(67),
        },
    )
    .unwrap();

    // Construct all algorithms
    let aline: Aline = import("tests/config_sample_aline.toml").unwrap();
    let bisim: BiSim = import("tests/config_sample_bisim.toml").unwrap();
    let editex: Editex = import("tests/config_sample_editex.toml").unwrap();
    let jaro_winkler: JaroWinkler = import("tests/config_sample_jaro_winkler.toml").unwrap();
    let levenshtein: Levenshtein = import("tests/config_sample_levenshtein.toml").unwrap();
    let gram2_1_1: NGram = NGram::try_new(2, 1, 1, false, NGramMetric::Dice).unwrap();
    let gram2_2_2: NGram = NGram::try_new(2, 2, 2, false, NGramMetric::Dice).unwrap();
    let gram3_1_1: NGram = NGram::try_new(3, 1, 1, false, NGramMetric::Dice).unwrap();
    let gram3_2_2: NGram = NGram::try_new(3, 2, 2, false, NGramMetric::Dice).unwrap();
    let prefix: Prefix = Prefix::new(false);

    let all_train = ScoreMatrix::from_slice(
        vec![
            Box::new(aline.clone()),
            Box::new(bisim.clone()),
            Box::new(editex.clone()),
            Box::new(jaro_winkler.clone()),
            Box::new(levenshtein.clone()),
            Box::new(gram2_1_1.clone()),
            Box::new(gram2_2_2.clone()),
            Box::new(gram3_1_1.clone()),
            Box::new(gram3_2_2.clone()),
            Box::new(prefix.clone()),
        ],
        &train,
        true,
    )
    .unwrap();

    let all_test = ScoreMatrix::from_slice(
        vec![
            Box::new(aline.clone()),
            Box::new(bisim.clone()),
            Box::new(editex.clone()),
            Box::new(jaro_winkler.clone()),
            Box::new(levenshtein.clone()),
            Box::new(gram2_1_1.clone()),
            Box::new(gram2_2_2.clone()),
            Box::new(gram3_1_1.clone()),
            Box::new(gram3_2_2.clone()),
            Box::new(prefix.clone()),
        ],
        &train,
        true,
    )
    .unwrap();

    all_train.export("D_train.csv").unwrap();
    all_test.export("D_test.csv").unwrap();
}
