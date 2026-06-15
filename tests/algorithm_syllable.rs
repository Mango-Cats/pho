use pho::{
    algorithms::{Algorithm, Syllable},
    utils::io::import,
};

const TOML_PATH: &str = "algorithm_configs/eng/syllable.toml";

fn load() -> Syllable {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

#[test]
fn identical_strings_score_one() {
    let config = load();
    assert!((config.similarity("sildenafil", "sildenafil").unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn score_in_range() {
    let config = load();
    let score = config.similarity("sildenafil", "tadalafil").unwrap();
    assert!((0.0..=1.0).contains(&score));
}

#[test]
fn shared_suffix_class_scores_higher() {
    let config = load();
    // -afil drugs share final syllables.
    let same_class = config.similarity("sildenafil", "tadalafil").unwrap();
    let diff_class = config.similarity("sildenafil", "metoprolol").unwrap();
    assert!(same_class >= diff_class, "same_class={same_class}, diff_class={diff_class}");
}

#[test]
fn single_syllable_words_exact_match() {
    let config = load();
    assert!((config.similarity("gly", "gly").unwrap() - 1.0).abs() < 1e-6);
    assert!((config.similarity("gly", "met").unwrap() - 0.0).abs() < 1e-6);
}
