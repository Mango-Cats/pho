use pho::{
    algorithms::{Algorithm, Prefix},
    utils::io::import,
};

const TOML_PATH: &str = "tests/config_sample_prefix.toml";

fn load() -> Prefix {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

#[test]
fn identical_similarity_is_one() {
    let config = load();
    let score = config.similarity("prefix", "prefix").unwrap();
    assert!((score - 1.0).abs() < 1e-6);
}

#[test]
fn shared_prefix_scores_higher_than_unrelated_strings() {
    let config = Prefix::new(true);

    let close = config.similarity("prefix", "prelude").unwrap();
    let far = config.similarity("prefix", "banana").unwrap();

    assert!((0.0..=1.0).contains(&close));
    assert!((0.0..=1.0).contains(&far));
    assert!(close > far, "expected shared prefix to score higher");
}

#[test]
fn respects_case_insensitive_setting() {
    let config = Prefix::new(true);
    let score = config.similarity("Prefix", "prefix").unwrap();
    assert!((score - 1.0).abs() < 1e-6);
}