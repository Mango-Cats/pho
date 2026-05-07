use pho::algorithms::{
    Aline, BiSim, CharTfIdf, DoubleMetaphone, Editex, JaroWinkler, Keyboard, LCS, LCSuf,
    Levenshtein, Metaphone, NGram, NGramMetric, NeedlemanWunsch, Prefix, SmithWaterman, Soundex,
    Syllable,
};
use pho::dataset::{Row, ScoreMatrix};
use pho::utils::io::{import, read_csv_as};

fn main() {
    // Algorithms
    // ...
    //  Construct all algorithm families defined in pho.
    let aline: Aline = import("tests/config_sample_aline.toml").unwrap();
    let bisim: BiSim = import("tests/config_sample_bisim.toml").unwrap();
    let double_metaphone: DoubleMetaphone =
        import("tests/config_sample_double_metaphone.toml").unwrap();
    let editex: Editex = import("tests/config_sample_editex.toml").unwrap();
    let jaro_winkler: JaroWinkler = import("tests/config_sample_jaro_winkler.toml").unwrap();
    let keyboard: Keyboard = import("tests/config_sample_keyboard.toml").unwrap();
    let lcs: LCS = LCS::new(false);
    let lcsuf: LCSuf = LCSuf::new(false);
    let levenshtein: Levenshtein = import("tests/config_sample_levenshtein.toml").unwrap();
    let metaphone: Metaphone = import("tests/config_sample_metaphone.toml").unwrap();
    let needleman_wunsch: NeedlemanWunsch =
        import("tests/config_sample_needleman_wunsch.toml").unwrap();
    let gram2_1_1: NGram = NGram::try_new(2, 1, 1, false, NGramMetric::Dice).unwrap();
    let gram2_2_2: NGram = NGram::try_new(2, 2, 2, false, NGramMetric::Dice).unwrap();
    let gram3_1_1: NGram = NGram::try_new(3, 1, 1, false, NGramMetric::Dice).unwrap();
    let gram3_2_2: NGram = NGram::try_new(3, 2, 2, false, NGramMetric::Dice).unwrap();
    let prefix: Prefix = Prefix::new(false);
    let smith_waterman: SmithWaterman = import("tests/config_sample_smith_waterman.toml").unwrap();
    let soundex: Soundex = import("tests/config_sample_soundex.toml").unwrap();
    let syllable: Syllable = import("tests/config_sample_syllable.toml").unwrap();
    let char_tfidf: CharTfIdf = CharTfIdf::try_new(2, false, false).unwrap();

    // Reading a CSV
    // ...
    //  Now let's read a CSV file that contains drug name pairs,
    //  their phonetic transcriptions, and their label (0: Negative;
    //  1: Positive/LASA).
    let all_data: Vec<Row> = read_csv_as("<yourfile>.csv", None).unwrap();

    // ScoreMatrix Construction
    // ...
    //  Now we can precompute all pairs given all the algorithms
    //  and variants defined, and append the length-based features.
    //
    //  Warning: this will take a long time and this will export a
    //  very large file.
    let include_word_level_features = true;
    let all_sm = ScoreMatrix::from_slice(
        vec![
            Box::new(aline.clone()),
            Box::new(bisim.clone()),
            Box::new(double_metaphone.clone()),
            Box::new(editex.clone()),
            Box::new(jaro_winkler.clone()),
            Box::new(keyboard.clone()),
            Box::new(lcs.clone()),
            Box::new(lcsuf.clone()),
            Box::new(levenshtein.clone()),
            Box::new(metaphone.clone()),
            Box::new(needleman_wunsch.clone()),
            Box::new(gram2_1_1.clone()),
            Box::new(gram2_2_2.clone()),
            Box::new(gram3_1_1.clone()),
            Box::new(gram3_2_2.clone()),
            Box::new(prefix.clone()),
            Box::new(smith_waterman.clone()),
            Box::new(soundex.clone()),
            Box::new(syllable.clone()),
            Box::new(char_tfidf.clone()),
        ],
        &all_data,
        include_word_level_features,
        true,
    )
    .unwrap();

    all_sm.export("mass_precomputed.parquet").unwrap();
}
