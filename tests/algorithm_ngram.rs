use pho::{
    algorithms::{Algorithm, NGram, NGramMetric},
    utils::io::import,
};

const TOML_PATH: &str = "tests/config_sample_ngram.toml";

fn load() -> NGram {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

fn assert_unit_interval(score: f32) {
    assert!((0.0..=1.0).contains(&score), "score out of range: {score}");
}

#[test]
fn identical_similarity_is_one() {
    let config = load();
    let score = config.similarity("banana", "banana").unwrap();
    assert!((score - 1.0).abs() < 1e-6);
}

#[test]
fn dice_jaccard_overlap_and_tversky_stay_in_range() {
    let dice = NGram::try_new(2, 1, 1, true, NGramMetric::Dice).unwrap();
    let jaccard = NGram::try_new(2, 1, 1, true, NGramMetric::Jaccard).unwrap();
    let overlap = NGram::try_new(2, 1, 1, true, NGramMetric::Overlap).unwrap();
    let tversky = NGram::try_new(
        2,
        1,
        1,
        true,
        NGramMetric::Tversky {
            alpha: 0.7,
            beta: 0.3,
        },
    )
    .unwrap();

    for score in [
        dice.similarity("night", "nacht").unwrap(),
        jaccard.similarity("night", "nacht").unwrap(),
        overlap.similarity("night", "nacht").unwrap(),
        tversky.similarity("night", "nacht").unwrap(),
    ] {
        assert_unit_interval(score);
    }
}

#[test]
fn cosine_similarity_is_in_range_and_rewards_closer_strings() {
    let config = NGram::try_new(3, 2, 2, true, NGramMetric::Cosine).unwrap();

    let close = config.similarity("phonetics", "fonetics").unwrap();
    let far = config.similarity("phonetics", "banana").unwrap();

    assert_unit_interval(close);
    assert_unit_interval(far);
    assert!(
        close > far,
        "expected close pair score to exceed far pair score"
    );
}

#[test]
fn name_includes_ngram_and_padding_config() {
    let config = NGram::try_new(3, 2, 1, true, NGramMetric::Dice).unwrap();
    assert_eq!(config.name(), "NGram_3_2_1");
}
