use pho::{
    algorithms::{Algorithm, Soundex},
    utils::io::import,
};

const TOML_PATH: &str = "tests/config_sample_soundex.toml";

fn load() -> Soundex {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

#[test]
fn identical_strings_score_one() {
    let config = load();
    assert!((config.similarity("Robert", "Robert").unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn soundalike_names_score_one() {
    let config = load();
    // "Robert" and "Rupert" share code R163.
    assert!((config.similarity("Robert", "Rupert").unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn unrelated_names_score_zero() {
    let config = load();
    // "Smith" (S530) and "Johnson" (J525) have different codes.
    assert!((config.similarity("Smith", "Johnson").unwrap() - 0.0).abs() < 1e-6);
}

#[test]
fn case_insensitive_soundalike() {
    let config = load();
    assert!((config.similarity("robert", "RUPERT").unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn soft_mode_gives_partial_credit() {
    use pho::algorithms::soundex::config::SoundexMode;
    let config = Soundex::new(true, SoundexMode::Soft);
    // "Herman" H655 vs "Hartman" H635 — differ only in one digit position.
    let score = config.similarity("Herman", "Hartman").unwrap();
    assert!(score > 0.0 && score < 1.0);
}
