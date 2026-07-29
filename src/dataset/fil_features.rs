//! Filipino (Tagalog) nativization indicator features.
//!
//! These mirror the `fil_*` columns in the `walter` pipeline's
//! `src/pipeline/features.py`, which derives them from the same nativized
//! spelling produced by the `tagabaybay` adapter (there, reached over a
//! subprocess/JSONL protocol to `tbb-cli`; here, linked in-process). They are
//! indicators of whether a Filipino speaker would hear the two names as
//! structurally alike after loanword nativization, not similarity scores.
//!
//! G2P-based adaptation (`AdapterConfig::g2p_unpredictable_variants`) is left
//! disabled: it shells out to a `uv run` Python/espeak-ng subprocess on first
//! use, which would make `phoc` depend on a Python toolchain being present.
//! Nativization instead falls back to `tagabaybay`'s non-G2P orthographic
//! rules, which cover the predictable cases. This means a handful of
//! ambiguous-vowel words may nativize slightly differently here than through
//! walter's `tbb-cli`, which enables G2P by default.

use tagabaybay::adaptation::adapter::Adapter;
use tagabaybay::configs::AdapterConfig;
use tagabaybay::grapheme::filipino::graphemes_to_string;

pub const FIL_FEATURES: [&str; 5] = [
    "fil_vowel_skeleton_match",
    "fil_penult_vowel_match",
    "fil_onset_match",
    "fil_coda_match",
    "fil_phonetic_equal",
];

fn nativize(word: &str) -> String {
    let config = AdapterConfig::new().set_g2p_unpredictable_variants(false);
    let mut adapter = Adapter::new_with_config(config);
    match adapter.adaptation(word) {
        Ok(graphemes) => graphemes_to_string(&graphemes),
        Err(_) => sanitize(word),
    }
}

/// Letters-only, lowercased skeleton used when the adapter can't nativize a
/// word, so one odd input degrades gracefully instead of aborting a run.
fn sanitize(word: &str) -> String {
    word.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_lowercase() || *c == 'ñ')
        .collect()
}

/// Collapse a nativized spelling's vowel letters to their 3-vowel skeleton
/// (Filipino's native vowel inventory raises unstressed e/o to i/u).
fn vowel_skeleton(nativized: &str) -> String {
    nativized
        .chars()
        .filter_map(|c| match c {
            'a' | 'i' | 'u' => Some(c),
            'e' => Some('i'),
            'o' => Some('u'),
            _ => None,
        })
        .collect()
}

/// The penultimate (default-stress) vowel, falling back to the only vowel in
/// a monosyllable; `None` if there is no vowel at all.
fn penult_vowel(vowels: &str) -> Option<char> {
    let chars: Vec<char> = vowels.chars().collect();
    if chars.len() >= 2 {
        Some(chars[chars.len() - 2])
    } else {
        chars.last().copied()
    }
}

fn bool_feature(matched: bool) -> f32 {
    if matched { 1.0 } else { 0.0 }
}

pub(super) fn feature_values(left: &str, right: &str, names: &[&'static str]) -> Vec<f32> {
    let (nat_left, nat_right) = (nativize(left), nativize(right));
    let (vowels_left, vowels_right) = (vowel_skeleton(&nat_left), vowel_skeleton(&nat_right));

    names
        .iter()
        .map(|name| match *name {
            "fil_onset_match" => bool_feature(matches!(
                (nat_left.chars().next(), nat_right.chars().next()),
                (Some(a), Some(b)) if a == b
            )),
            "fil_coda_match" => bool_feature(matches!(
                (nat_left.chars().last(), nat_right.chars().last()),
                (Some(a), Some(b)) if a == b
            )),
            "fil_vowel_skeleton_match" => bool_feature(vowels_left == vowels_right),
            "fil_penult_vowel_match" => bool_feature(matches!(
                (penult_vowel(&vowels_left), penult_vowel(&vowels_right)),
                (Some(a), Some(b)) if a == b
            )),
            "fil_phonetic_equal" => bool_feature(nat_left == nat_right),
            _ => unreachable!("unknown fil feature name"),
        })
        .collect()
}
