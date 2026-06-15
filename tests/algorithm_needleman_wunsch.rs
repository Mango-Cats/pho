use pho::{
    algorithms::{Algorithm, NeedlemanWunsch},
    utils::io::import,
};

const TOML_PATH: &str = "algorithm_configs/eng/needleman_wunsch.toml";

fn load() -> NeedlemanWunsch {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

#[test]
fn identical_strings_score_one() {
    let config = load();
    assert!((config.similarity("captopril", "captopril").unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn score_in_range() {
    let config = load();
    let score = config.similarity("enalapril", "ramipril").unwrap();
    assert!((0.0..=1.0).contains(&score));
}

#[test]
fn phonetically_similar_substitutions_score_higher() {
    let config = load();
    // f/v are in the same substitution group → "afil" vs "avil" scores higher than random.
    let similar = config.similarity("sildenafil", "sildenaVil").unwrap();
    let dissimilar = config.similarity("sildenafil", "metformin").unwrap();
    assert!(similar > dissimilar, "similar={similar}, dissimilar={dissimilar}");
}

#[test]
fn closer_pair_scores_higher() {
    let config = load();
    let close = config.similarity("lisinopril", "enalapril").unwrap();
    let far = config.similarity("lisinopril", "metformin").unwrap();
    assert!(close > far, "close={close}, far={far}");
}
