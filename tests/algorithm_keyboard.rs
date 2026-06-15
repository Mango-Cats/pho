use pho::{
    algorithms::{Algorithm, Keyboard},
    utils::io::import,
};

const TOML_PATH: &str = "algorithm_configs/eng/keyboard.toml";

fn load() -> Keyboard {
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
fn empty_strings_score_one() {
    let config = load();
    assert!((config.similarity("", "").unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn one_empty_scores_zero() {
    let config = load();
    assert!((config.similarity("metformin", "").unwrap() - 0.0).abs() < 1e-6);
}

#[test]
fn adjacent_key_typo_scores_higher_than_distant() {
    let config = load();
    // 's' and 'd' are adjacent; 's' and 'q' are also close; 's' and 'm' are far.
    let close = config.similarity("sale", "dale").unwrap(); // s→d: adjacent
    let far = config.similarity("sale", "male").unwrap();   // s→m: far
    assert!(close > far, "close={close}, far={far}");
}

#[test]
fn distance_and_similarity_are_complementary() {
    let config = load();
    let d = config.normalized_distance("lisinopril", "lisanopril").unwrap();
    let s = config.similarity("lisinopril", "lisanopril").unwrap();
    assert!((d + s - 1.0).abs() < 1e-5, "d={d}, s={s}");
}

#[test]
fn score_in_range() {
    let config = load();
    let score = config.similarity("atenolol", "metoprolol").unwrap();
    assert!((0.0..=1.0).contains(&score));
}
