use pho::{
    algorithms::editex::{self, config::EditexConfig},
    config_io::parse_toml_file,
};

const TOML_PATH: &str = "tests/config_sample_editex.toml";

fn load() -> EditexConfig {
    match parse_toml_file(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

fn assert_approx(actual: f32, expected: f32, tol: f32, word1: &str, word2: &str) {
    let diff = (actual - expected).abs();
    assert!(
        diff <= tol,
        "{word1} ~ {word2}: expected {expected} ± {tol}, got {actual} (diff {diff})"
    );
}

#[test]
fn matches_reference_editex_distances() {
    let config = load();
    let tol = 1e-6;

    const CASES: &[(&str, &str, f32)] = &[
        ("Smith", "Smyth", 1.0),
        ("Catherine", "Katherine", 1.0),
        ("Brian", "Bryan", 1.0),
        ("Calendar", "Calender", 1.0),
        ("Similarity", "Simularity", 1.0),
        ("Color", "Colour", 1.0),
        ("Relevant", "Relevante", 2.0),
        ("Knight", "Night", 2.0),
        ("Stephen", "Steven", 3.0),
        ("Paxil", "Taxol", 3.0),
        ("Accept", "Except", 3.0),
        ("Zantac", "Xanax", 5.0),
        ("Physics", "Fizziks", 6.0),
        ("Apple", "Banana", 9.0),
    ];

    for &(w1, w2, expected) in CASES {
        let actual = editex::distance(w1, w2, &config).unwrap();
        assert_approx(actual, expected, tol, w1, w2);
    }
}

#[test]
fn wrapper_similarity_is_in_range() {
    let config = load();
    let sim = editex::similarity("Smith", "Smyth", &config).unwrap();
    assert!((0.0..=1.0).contains(&sim));
}
