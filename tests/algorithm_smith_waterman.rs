use pho::{
    algorithms::{Algorithm, SmithWaterman},
    utils::io::import,
};

const TOML_PATH: &str = "tests/config_sample_smith_waterman.toml";

fn load() -> SmithWaterman {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

#[test]
fn identical_strings_score_one() {
    let config = load();
    assert!((config.similarity("lisinopril", "lisinopril").unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn empty_strings_score_one() {
    let config = load();
    assert!((config.similarity("", "").unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn one_empty_scores_zero() {
    let config = load();
    assert!((config.similarity("lisinopril", "").unwrap() - 0.0).abs() < 1e-6);
}

#[test]
fn shared_substring_scores_above_zero() {
    let config = load();
    // "metoprolol" and "propranolol" share "-olol" locally.
    let score = config.similarity("metoprolol", "propranolol").unwrap();
    assert!(score > 0.0, "score={score}");
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
    let close = config.similarity("sildenafil", "tadalafil").unwrap();
    let far = config.similarity("sildenafil", "metformin").unwrap();
    assert!(close > far, "close={close}, far={far}");
}
