use pho::{
    algorithms::{Algorithm, Levenshtein},
    utils::io::import,
};

const TOML_PATH: &str = "algorithm_configs/eng/levenshtein.toml";
const CONSONANT_TOML_PATH: &str = "algorithm_configs/eng/levenshtein_consonant.toml";

fn load() -> Levenshtein {
    match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

fn load_consonant() -> Levenshtein {
    match import(CONSONANT_TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {CONSONANT_TOML_PATH}: {e}."),
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

#[test]
fn edit_operation_counts_matches_hand_worked_alignment() {
    let config = load();

    // kitten -> sitting: substitute k->s, substitute e->i, insert g.
    let (substitutions, insertions, deletions) =
        config.edit_operation_counts("kitten", "sitting").unwrap();
    assert_eq!((substitutions, insertions, deletions), (2, 1, 0));

    // Total operation count should match the (uniform-cost) distance.
    let distance = config.distance("kitten", "sitting").unwrap();
    assert_eq!(distance, (substitutions + insertions + deletions) as f32);
}

#[test]
fn edit_operation_counts_pure_deletion() {
    let config = load();

    let (substitutions, insertions, deletions) = config.edit_operation_counts("abc", "").unwrap();
    assert_eq!((substitutions, insertions, deletions), (0, 0, 3));
}

#[test]
fn edit_operation_counts_pure_insertion() {
    let config = load();

    let (substitutions, insertions, deletions) = config.edit_operation_counts("", "abc").unwrap();
    assert_eq!((substitutions, insertions, deletions), (0, 3, 0));
}

#[test]
fn edit_operation_counts_identical_strings_are_all_zero() {
    let config = load();

    let (substitutions, insertions, deletions) =
        config.edit_operation_counts("kitten", "kitten").unwrap();
    assert_eq!((substitutions, insertions, deletions), (0, 0, 0));
}

#[test]
fn consonants_only_ignores_vowel_only_differences() {
    let config = load_consonant();

    // "color" -> consonants "clr"; "colour" -> consonants "clr" (u is a vowel).
    assert_eq!(config.similarity("color", "colour").unwrap(), 1.0);
}

#[test]
fn consonants_only_treats_y_as_a_consonant() {
    let config = load_consonant();

    // "sky" -> consonants "sky" (y kept); "ski" -> consonants "sk" (i dropped).
    // These are not equal, so similarity must be less than 1.0.
    assert!(config.similarity("sky", "ski").unwrap() < 1.0);
}

#[test]
fn consonants_only_still_scores_consonant_substitutions() {
    let config = load_consonant();

    let close = config.similarity("cat", "cot").unwrap(); // consonants "ct" vs "ct"
    let far = config.similarity("cat", "cap").unwrap(); // consonants "ct" vs "cp"

    assert_eq!(close, 1.0);
    assert!(far < 1.0);
}

#[test]
fn consonants_only_both_all_vowels_score_one() {
    let config = load_consonant();
    assert_eq!(config.similarity("aeiou", "aeiou").unwrap(), 1.0);
}

#[test]
fn consonants_only_respects_case_insensitive_toggle() {
    let content = std::fs::read_to_string(CONSONANT_TOML_PATH).unwrap();
    let content = content.replace("case_insensitive = false", "case_insensitive = true");
    let insensitive: Levenshtein = toml::from_str(&content).unwrap();

    assert!(load_consonant().similarity("CAT", "cat").unwrap() < 1.0);
    assert_eq!(insensitive.similarity("CAT", "cat").unwrap(), 1.0);
}

#[test]
fn plain_levenshtein_does_not_ignore_vowels() {
    let config = load();

    // Without consonants_only, "color" vs "colour" differs by an inserted "u".
    assert!(config.similarity("color", "colour").unwrap() < 1.0);
}
