use pho::{
    algorithms::{Algorithm, Levenshtein},
    utils::io::import,
};

const TOML_PATH: &str = "tests/config_sample_levenshtein.toml";

fn load() -> Levenshtein {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

#[test]
fn identical_strings_have_zero_distance() {
    let config = load();
    assert_eq!(config.distance("kitten", "kitten").unwrap(), 0.0);
    assert_eq!(config.normalized_distance("kitten", "kitten").unwrap(), 0.0);
    assert!((config.similarity("kitten", "kitten").unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn distance_and_similarity_move_in_opposite_directions() {
    let config = load();

    let close_distance = config.distance("kitten", "sitting").unwrap();
    let far_distance = config.distance("kitten", "banana").unwrap();
    let close_similarity = config.similarity("kitten", "sitting").unwrap();
    let far_similarity = config.similarity("kitten", "banana").unwrap();

    assert!(close_distance < far_distance);
    assert!(close_similarity > far_similarity);
    assert!((0.0..=1.0).contains(&config.normalized_distance("kitten", "sitting").unwrap()));
    assert!((0.0..=1.0).contains(&config.normalized_distance("kitten", "banana").unwrap()));
}
