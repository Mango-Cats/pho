use pho::{
    algorithms::{Algorithm, JaroWinkler},
    utils::io::import,
};

const TOML_PATH: &str = "tests/config_sample_jaro_winkler.toml";

fn load() -> JaroWinkler {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

#[test]
fn identical_strings_have_zero_distance() {
    let config = load();

    assert_eq!(config.distance("dixon", "dixon").unwrap(), 0.0);
    assert_eq!(config.normalized_distance("dixon", "dixon").unwrap(), 0.0);
    assert!((config.similarity("dixon", "dixon").unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn closer_words_have_smaller_distance() {
    let config = load();

    let close_distance = config.distance("dixon", "dicksonx").unwrap();
    let far_distance = config.distance("dixon", "banana").unwrap();
    let close_similarity = config.similarity("dixon", "dicksonx").unwrap();
    let far_similarity = config.similarity("dixon", "banana").unwrap();

    assert!((0.0..=1.0).contains(&close_distance));
    assert!((0.0..=1.0).contains(&far_distance));
    assert!(close_distance < far_distance);
    assert!(close_similarity > far_similarity);
}

#[test]
fn respects_case_insensitive_setting() {
    let config = JaroWinkler::try_new(0.1, 4, true).unwrap();
    assert!((config.distance("Dixon", "dixon").unwrap() - 0.0).abs() < 1e-6);
}