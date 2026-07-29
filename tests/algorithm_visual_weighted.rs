use pho::{
    algorithms::{Algorithm, VisualWeighted},
    utils::io::import,
};

const TOML_PATH: &str = "algorithm_configs/eng/visual_weighted.toml";

fn load() -> VisualWeighted {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

#[test]
fn identical_similarity_is_one() {
    let config = load();
    let sim = config.similarity("bad", "bad").unwrap();
    assert!((sim - 1.0).abs() < 1e-6);
}

#[test]
fn visually_confusable_pair_scores_higher_than_unrelated_pair() {
    let config = load();
    let close = config.similarity("bat", "dat").unwrap();
    let far = config.similarity("bat", "zat").unwrap();

    assert!((0.0..=1.0).contains(&close));
    assert!((0.0..=1.0).contains(&far));
    assert!(
        close > far,
        "expected visually confusable pair score to exceed unrelated pair score"
    );
}

#[test]
fn distance_and_normalized_distance_are_available() {
    let config = load();

    let close_distance = config.distance("bat", "dat").unwrap();
    let far_distance = config.distance("bat", "zat").unwrap();
    let close_normalized = config.normalized_distance("bat", "dat").unwrap();
    let far_normalized = config.normalized_distance("bat", "zat").unwrap();

    assert!((0.0..=1.0).contains(&close_normalized));
    assert!((0.0..=1.0).contains(&far_normalized));
    assert!(close_distance < far_distance);
    assert!(close_normalized < far_normalized);
}

#[test]
fn ignores_non_alphabet_characters() {
    let config = load();

    let plain = config.similarity("bat", "dat").unwrap();
    let noisy = config.similarity("b!a1t*", "d_at#42").unwrap();

    assert!((plain - noisy).abs() < 1e-6);
}

#[test]
fn edit_operation_counts_matches_hand_worked_alignment() {
    let config = load();

    // "bat" -> "dot": substitute b->d, substitute a->o.
    let (substitutions, insertions, deletions) = config.edit_operation_counts("bat", "dot").unwrap();
    assert_eq!((substitutions, insertions, deletions), (2, 0, 0));
}

#[test]
fn edit_operation_counts_identical_strings_are_all_zero() {
    let config = load();

    let (substitutions, insertions, deletions) = config.edit_operation_counts("bat", "bat").unwrap();
    assert_eq!((substitutions, insertions, deletions), (0, 0, 0));
}

#[test]
fn case_sensitive_by_default() {
    let config = load();

    let same_case = config.similarity("bat", "dat").unwrap();
    let cross_case = config.similarity("bat", "Dat").unwrap();

    assert!(
        same_case > cross_case,
        "expected same-case confusable pair to score higher than a cross-case pair, since the \
         source study never rated cross-case pairs"
    );
}

#[test]
fn case_insensitive_toggle_folds_case() {
    let content = std::fs::read_to_string(TOML_PATH).unwrap();
    let content = content.replace("case_insensitive = false", "case_insensitive = true");
    let config: VisualWeighted = toml::from_str(&content).unwrap();

    let similarity = config.similarity("BAT", "dat").unwrap();
    assert!((0.0..=1.0).contains(&similarity));
    assert!((similarity - config.similarity("bat", "dat").unwrap()).abs() < 1e-6);
}
