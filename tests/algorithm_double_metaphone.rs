use pho::{
    algorithms::{Algorithm, DoubleMetaphone},
    utils::io::import,
};

const TOML_PATH: &str = "tests/config_sample_double_metaphone.toml";

fn load() -> DoubleMetaphone {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

#[test]
fn identical_strings_score_one() {
    let config = load();
    assert!((config.similarity("metformin", "metformin").unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn score_is_in_range() {
    let config = load();
    let score = config.similarity("lisinopril", "captopril").unwrap();
    assert!((0.0..=1.0).contains(&score));
}

#[test]
fn closer_names_score_higher() {
    let config = load();
    let close = config.similarity("atorvastatin", "rosuvastatin").unwrap();
    let far = config.similarity("atorvastatin", "metformin").unwrap();
    assert!(close >= far, "close={close}, far={far}");
}
