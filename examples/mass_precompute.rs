use pho::algorithms::{Editex, JaroWinkler, Levenshtein, NGram, NGramMetric, Prefix};
use pho::dataset::{Row, ScoreMatrix};
use pho::{
    algorithms::{Aline, BiSim},
    utils::io::{import, read_csv_as},
};

fn main() {
    // Algorithms
    // ...
    //  Construct all algorithms defined in pho and even initialize
    //  variants of the NGram algorithms.
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

    // Reading a CSV
    // ...
    //  Now let's read a CSV file that contains drug name pairs,
    //  their phonetic transcriptions, and their label (0: Negative;
    //  1: Positive/LASA).
    let all_data: Vec<Row> = read_csv_as("<yourfile>.csv", None).unwrap();

    // ScoreMatrix Construction
    //  ...
    //  Now we can precompute all pairs given all the algorithms
    //  and variants defined.
    //
    //  Warning: this will take a long time and this will export a
    //  very large file.
    let all_sm = ScoreMatrix::from_slice(
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
        &all_data,
        true,
    )
    .unwrap();

    all_sm.export("mass_precomputed.arrow").unwrap();
}
