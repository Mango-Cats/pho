use pho::{
    algorithms::{Algorithm, BiSim},
    utils::io::import,
};

const TOML_PATH: &str = "tests/config_sample_bisim.toml";

fn load() -> BiSim {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

#[test]
fn identical_similarity_is_one() {
    let config = load();
    let score = config.similarity("Zantac", "Zantac").unwrap();
    assert!((score - 1.0).abs() < 1e-6);
}

#[test]
fn first_letter_repetition_and_ordering_affect_score() {
    let config = BiSim::try_new(true).unwrap();

    let close = config.similarity("Zantac", "Zantax").unwrap();
    let farther = config.similarity("Zantac", "banana").unwrap();

    assert!((0.0..=1.0).contains(&close));
    assert!((0.0..=1.0).contains(&farther));
    assert!(close > farther, "expected closer spelling to score higher");
}

#[test]
fn respects_case_insensitive_setting() {
    let config = BiSim::try_new(true).unwrap();
    let score = config.similarity("Zantac", "zantac").unwrap();
    assert!((score - 1.0).abs() < 1e-6);
}
