use pho::algorithms::{Algorithm, LCSubstring};

#[test]
fn identical_strings_score_one() {
    let algo = LCSubstring::new(false);
    assert_eq!(algo.similarity("hello", "hello").unwrap(), 1.0);
}

#[test]
fn both_empty_scores_one() {
    let algo = LCSubstring::new(false);
    assert_eq!(algo.similarity("", "").unwrap(), 1.0);
}

#[test]
fn one_empty_scores_zero() {
    let algo = LCSubstring::new(false);
    assert_eq!(algo.similarity("abc", "").unwrap(), 0.0);
}

#[test]
fn finds_longest_contiguous_run_not_subsequence() {
    let algo = LCSubstring::new(false);

    // "axbxcx" and "abcxxx" share the subsequence "abc" (LCS = 3) but their
    // longest contiguous shared run is just "xx" (length 2).
    let score = algo.similarity("axbxcx", "abcxxx").unwrap();
    assert!((score - 2.0 / 6.0).abs() < 1e-6);
}

#[test]
fn common_substring_at_different_positions() {
    let algo = LCSubstring::new(false);

    // Longest common substring of "abcdef" and "xxcdefyy" is "cdef" (length 4),
    // normalized by the longer string's length (8).
    let score = algo.similarity("abcdef", "xxcdefyy").unwrap();
    assert!((score - 4.0 / 8.0).abs() < 1e-6);
}

#[test]
fn case_insensitive_toggle() {
    let sensitive = LCSubstring::new(false);
    let insensitive = LCSubstring::new(true);

    assert!(sensitive.similarity("ABC", "abc").unwrap() < 1.0);
    assert_eq!(insensitive.similarity("ABC", "abc").unwrap(), 1.0);
}
