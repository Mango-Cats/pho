use pho::{
    algorithms::{Algorithm, CharTfIdf},
    utils::io::import,
};

const TOML_PATH: &str = "tests/config_sample_tfidf.toml";

fn load() -> CharTfIdf {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

#[test]
fn identical_strings_score_one() {
    let config = load();
    assert!((config.similarity("amlodipine", "amlodipine").unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn score_in_range() {
    let config = load();
    let score = config.similarity("amlodipine", "nifedipine").unwrap();
    assert!((0.0..=1.0).contains(&score));
}

#[test]
fn closer_pair_scores_higher() {
    let config = load();
    let close = config.similarity("amlodipine", "nifedipine").unwrap();
    let far = config.similarity("amlodipine", "metformin").unwrap();
    assert!(close > far, "close={close}, far={far}");
}

#[test]
fn empty_strings_score_one() {
    let config = load();
    assert!((config.similarity("", "").unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn invalid_n_is_rejected() {
    assert!(CharTfIdf::try_new(0, true, true).is_err());
}
