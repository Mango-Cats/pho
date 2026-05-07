use pho::{
    algorithms::{Algorithm, Metaphone},
    utils::io::import,
};

const TOML_PATH: &str = "tests/config_sample_metaphone.toml";

fn load() -> Metaphone {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

#[test]
fn identical_strings_score_one() {
    let config = load();
    assert!((config.similarity("metoprolol", "metoprolol").unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn phonetically_close_names_score_higher_than_distant() {
    let config = load();
    let close = config.similarity("sildenafil", "tadalafil").unwrap();
    let far = config.similarity("sildenafil", "metoprolol").unwrap();
    assert!(close > far, "close={close}, far={far}");
}

#[test]
fn phonetic_variants_score_above_zero() {
    let config = load();
    // "Smith" and "Smythe" share phonetics.
    let score = config.similarity("Smith", "Smythe").unwrap();
    assert!(score > 0.5, "score={score}");
}

#[test]
fn score_in_range() {
    let config = load();
    let score = config.similarity("amlodipine", "nifedipine").unwrap();
    assert!((0.0..=1.0).contains(&score));
}
