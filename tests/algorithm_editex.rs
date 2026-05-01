use pho::{
    algorithms::{AlgorithmTrait, EditexAlgorithm, editex::config::EditexConfig},
    config_io::import,
};

const TOML_PATH: &str = "tests/config_sample_editex.toml";

fn load() -> EditexConfig {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

#[test]
fn identical_similarity_is_one() {
    let config = load();
    let algo = EditexAlgorithm::new(&config);
    let sim = algo.similarity("Smith", "Smith").unwrap();
    assert!((sim - 1.0).abs() < 1e-6);
}

#[test]
fn closer_words_score_higher() {
    let config = load();
    let algo = EditexAlgorithm::new(&config);
    let close = algo.similarity("Smith", "Smyth").unwrap();
    let far = algo.similarity("Smith", "Banana").unwrap();

    assert!((0.0..=1.0).contains(&close));
    assert!((0.0..=1.0).contains(&far));
    assert!(
        close > far,
        "expected close pair score to exceed far pair score"
    );
}
