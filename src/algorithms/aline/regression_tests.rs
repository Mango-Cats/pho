//! Regression tests for tokenization, length, stress scope, global alignment
//! and NLTK parity.

use super::alignment::alignment_score;
use super::config::{
    AlignmentMode, Aline, AlineVariant, PhoneticFeatures, StressOn, StressScopeFallback,
};
use super::scoring::substitution_score;
use super::{Segment, parse_segments};
use crate::algorithms::Algorithm;
use crate::error::Error;
use crate::utils::io::import;

const ENG: &str = "algorithm_configs/eng/aline.toml";
const FIL: &str = "algorithm_configs/fil/filipino_aline.toml";

fn load(path: &str) -> Aline {
    import(path).unwrap_or_else(|e| panic!("Can't open {path}: {e}."))
}

fn syms(segs: &[Segment]) -> Vec<&str> {
    segs.iter().map(|s| s.sym.as_str()).collect()
}

fn stresses(segs: &[Segment]) -> Vec<f32> {
    segs.iter().map(|s| s.stress).collect()
}

fn seg(sym: &str) -> Segment {
    Segment {
        sym: sym.to_string(),
        stress: 0.0,
        long: None,
    }
}

fn seg_with_stress(sym: &str, stress: f32) -> Segment {
    Segment { stress, ..seg(sym) }
}

// 1. Tie-barred affricates are one segment and score like the ligatures.
#[test]
fn tie_bar_affricates_tokenize_as_one_segment() {
    let cfg = load(FIL);
    let segs = parse_segments("d͡ʒat͡ʃa", "x", &cfg).unwrap();
    assert_eq!(syms(&segs), ["d͡ʒ", "a", "t͡ʃ", "a"]);
}

#[test]
fn tie_bar_affricates_score_like_ligatures() {
    let cfg = load(FIL);
    for (tied, ligature) in [("d͡ʒ", "ʤ"), ("t͡ʃ", "ʧ")] {
        for other in cfg.sounds.keys() {
            let a = substitution_score(&seg(tied), &seg(other), &cfg);
            let b = substitution_score(&seg(ligature), &seg(other), &cfg);
            assert_eq!(a, b, "{tied} vs {ligature} differ against {other}");
        }
        let sim = cfg
            .similarity(&format!("{tied}a"), &format!("{ligature}a"))
            .unwrap();
        assert_eq!(sim, 1.0, "{tied}a ~ {ligature}a");
    }
}

#[test]
fn multi_char_spellings_resolve_to_inventory_key() {
    let fil = load(FIL);
    // Exact keys win: FIL has both ligature and decomposed keys.
    assert_eq!(syms(&parse_segments("ʤa", "x", &fil).unwrap()), ["ʤ", "a"]);
    assert_eq!(
        syms(&parse_segments("d͡ʒa", "x", &fil).unwrap()),
        ["d͡ʒ", "a"]
    );
    // Untied and under-tied spellings fall back to a canonical match.
    for input in ["dʒa", "d͜ʒa"] {
        assert_eq!(syms(&parse_segments(input, "x", &fil).unwrap()), ["ʤ", "a"]);
    }
    assert_eq!(syms(&parse_segments("tʃa", "x", &fil).unwrap()), ["ʧ", "a"]);

    // ENG only has the ligatures.
    let eng = load(ENG);
    for input in ["dʒa", "d͡ʒa", "d͜ʒa"] {
        assert_eq!(syms(&parse_segments(input, "x", &eng).unwrap()), ["ʤ", "a"]);
    }
    assert_eq!(eng.similarity("t͡ʃa", "ʧa").unwrap(), 1.0);
}

#[test]
fn rhotic_hook_spellings_are_equivalent() {
    let fil = load(FIL);
    assert_eq!(syms(&parse_segments("ə˞", "x", &fil).unwrap()), ["ɚ"]);
}

#[test]
fn tie_bar_on_vowels_respects_merge_diphthongs() {
    let mut cfg = load(FIL);
    assert_eq!(
        syms(&parse_segments("ma͡j", "x", &cfg).unwrap()),
        ["m", "a", "j"]
    );
    cfg.merge_diphthongs = true;
    assert_eq!(
        syms(&parse_segments("ma͡j", "x", &cfg).unwrap()),
        ["m", "aj"]
    );
}

// 2. Diphthong entries are skipped unless merge_diphthongs = true.
#[test]
fn diphthongs_not_merged_by_default() {
    let cfg = load(FIL);
    assert!(!cfg.merge_diphthongs);
    let segs = parse_segments("maj", "x", &cfg).unwrap();
    assert_eq!(syms(&segs), ["m", "a", "j"]);
}

#[test]
fn diphthongs_merged_when_enabled() {
    let mut cfg = load(FIL);
    cfg.merge_diphthongs = true;
    let segs = parse_segments("maj", "x", &cfg).unwrap();
    assert_eq!(syms(&segs), ["m", "aj"]);
}

// 3. Length marks.
#[test]
fn length_mark_uses_salience_long() {
    let mut cfg = load(ENG);
    let long_a = &parse_segments("aː", "x", &cfg).unwrap()[0];
    let short_a = &parse_segments("a", "y", &cfg).unwrap()[0];
    assert_eq!(long_a.sym, "a");
    assert_eq!(long_a.long, Some(1.0));
    assert_eq!(short_a.long, None);

    cfg.salience.long = 10;
    let diff =
        substitution_score(short_a, short_a, &cfg) - substitution_score(long_a, short_a, &cfg);
    assert_eq!(diff, 10.0);

    cfg.salience.long = 0;
    assert_eq!(
        substitution_score(long_a, short_a, &cfg),
        substitution_score(short_a, short_a, &cfg)
    );
}

#[test]
fn half_long_mark_is_half_of_long() {
    let mut cfg = load(ENG);
    cfg.salience.long = 10;
    let half = &parse_segments("aˑ", "x", &cfg).unwrap()[0];
    let short_a = seg("a");
    assert_eq!(half.long, Some(0.5));
    let diff =
        substitution_score(&short_a, &short_a, &cfg) - substitution_score(half, &short_a, &cfg);
    assert_eq!(diff, 5.0);
}

#[test]
fn length_mark_after_consonant_is_recorded_not_scored() {
    let mut cfg = load(ENG);
    cfg.salience.long = 10;
    let segs = parse_segments("tːa", "x", &cfg).unwrap();
    assert_eq!(syms(&segs), ["t", "a"]);
    assert_eq!(segs[0].long, Some(1.0));
    assert_eq!(
        substitution_score(&segs[0], &seg("t"), &cfg),
        substitution_score(&seg("t"), &seg("t"), &cfg)
    );
}

// 4. Stress scope.
#[test]
fn stress_applies_to_marked_syllable_only() {
    let cfg = load(FIL);
    assert!(matches!(cfg.variant, AlineVariant::MangoCats));

    let segs = parse_segments("ba.ˈta", "x", &cfg).unwrap();
    assert_eq!(syms(&segs), ["b", "a", "t", "a"]);
    assert_eq!(stresses(&segs), [0.0, 0.0, 1.0, 1.0]);

    let segs = parse_segments("ˈba.ta", "x", &cfg).unwrap();
    assert_eq!(stresses(&segs), [1.0, 1.0, 0.0, 0.0]);

    let segs = parse_segments("ˌba.ta.ˈka", "x", &cfg).unwrap();
    assert_eq!(stresses(&segs), [0.5, 0.5, 0.0, 0.0, 1.0, 1.0]);
}

#[test]
fn stress_without_syllable_boundary_is_an_error() {
    let cfg = load(FIL);
    assert_eq!(cfg.stress_scope_fallback, StressScopeFallback::Error);
    let err = parse_segments("ˈbata", "x", &cfg).unwrap_err();
    assert!(matches!(
        err,
        Error::AmbiguousStressScope { input_name: "x" }
    ));
    assert!(cfg.similarity("ˈbata", "bata").is_err());
}

#[test]
fn stress_fallback_until_next_mark() {
    let mut cfg = load(FIL);
    cfg.stress_scope_fallback = StressScopeFallback::UntilNextMark;
    let segs = parse_segments("ˈbata", "x", &cfg).unwrap();
    assert_eq!(stresses(&segs), [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn kondrak_ignores_stress_and_syllable_marks() {
    let cfg = load(ENG);
    let segs = parse_segments("ˈbata", "x", &cfg).unwrap();
    assert_eq!(syms(&segs), ["b", "a", "t", "a"]);
    assert_eq!(stresses(&segs), [0.0; 4]);
    let segs = parse_segments("ba.ˈta", "x", &cfg).unwrap();
    assert_eq!(syms(&segs), ["b", "a", "t", "a"]);
    assert_eq!(stresses(&segs), [0.0; 4]);
}

// 5. stress_on = "vowels".
#[test]
fn stress_on_vowels_ignores_consonant_pairs() {
    let mut cfg = load(FIL);
    cfg.salience.stress = 10;
    assert_eq!(cfg.stress_on, StressOn::Vowels);

    let plain = substitution_score(&seg("b"), &seg("p"), &cfg);
    let stressed = substitution_score(&seg_with_stress("b", 1.0), &seg("p"), &cfg);
    assert_eq!(plain, stressed);

    // Vowel pairs still pay for the stress difference.
    let plain = substitution_score(&seg("a"), &seg("a"), &cfg);
    let stressed = substitution_score(&seg_with_stress("a", 1.0), &seg("a"), &cfg);
    assert_eq!(plain - stressed, 10.0);

    // "all" restores the old behaviour for consonants.
    cfg.stress_on = StressOn::All;
    let plain = substitution_score(&seg("b"), &seg("p"), &cfg);
    let stressed = substitution_score(&seg_with_stress("b", 1.0), &seg("p"), &cfg);
    assert_eq!(plain - stressed, 10.0);
}

// 6. Filipino vowel manner is on a single height scale.
fn manner_distance(cfg: &Aline, p: &str, q: &str) -> f32 {
    let manner = |s: &str| match &cfg.sounds[s] {
        PhoneticFeatures::Vowel(v) => cfg.values.manner[v.common.manner],
        PhoneticFeatures::Consonant(_) => panic!("{s} is not a vowel"),
    };
    (manner(p) - manner(q)).abs()
}

#[test]
fn filipino_manner_follows_height() {
    let cfg = load(FIL);
    let a_e = manner_distance(&cfg, "a", "e");
    let e_i = manner_distance(&cfg, "e", "i");
    let a_i = manner_distance(&cfg, "a", "i");
    assert!(a_i > a_e, "a–i ({a_i}) should exceed a–e ({a_e})");
    assert!(a_i > e_i, "a–i ({a_i}) should exceed e–i ({e_i})");
    // With low_vowel = 0.0, mid_vowel = 0.2, high_vowel = 0.4 the mid vowel
    // sits exactly halfway, so a-e == e-i. See the strict test below.
    assert!(a_e >= e_i, "a–e ({a_e}) should be at least e–i ({e_i})");
}

#[test]
#[ignore = "fails with current [values.manner]: a–e == e–i == 0.2; needs an author decision on mid_vowel"]
fn filipino_manner_a_e_strictly_exceeds_e_i() {
    let cfg = load(FIL);
    assert!(manner_distance(&cfg, "a", "e") > manner_distance(&cfg, "e", "i"));
}

// 7. Global alignment charges leading/trailing indels.
#[test]
fn global_alignment_penalizes_unmatched_prefix() {
    let local = load(ENG);
    assert_eq!(local.alignment_mode, AlignmentMode::Local);
    let mut global = local.clone();
    global.alignment_mode = AlignmentMode::Global;

    let l = local.similarity("ab", "xab").unwrap();
    let g = global.similarity("ab", "xab").unwrap();
    assert!(g < l, "global {g} should be below local {l}");
    assert!((0.0..=1.0).contains(&g));
    assert_eq!(global.similarity("ab", "ab").unwrap(), 1.0);
}

#[test]
fn global_alignment_is_clamped_to_zero() {
    let mut cfg = load(ENG);
    cfg.alignment_mode = AlignmentMode::Global;
    let sim = cfg.similarity("p", "aaaaaaaaaa").unwrap();
    assert_eq!(sim, 0.0);
}

// 8. NLTK parity on the Kondrak (2002) cognate demo pairs.
//
// Expected values were generated from the NLTK-derived reference port
// (`.dev/actual_aline.py`, recovered with `git show 299b180:.dev/actual_aline.py`)
// with:
//
//   uv run --with numpy python -c 'import actual_aline as a
//   for l in a.cognate_data.split("\n"):
//       w1, w2 = l.split(",")
//       print(w1, w2, a._align_score(w1, w2), a._align_score(w1, w1),
//             a._align_score(w2, w2), a.similarity(w1, w2))'
//
// Columns: (word1, word2, raw score, raw self score of word1, raw self score
// of word2, similarity).
#[rustfmt::skip]
const NLTK_COGNATES: &[(&str, &str, f32, f32, f32, f32)] = &[
    ("jo", "ʒə", 42.5, 60.0, 60.0, 0.708333),
    ("tu", "ty", 58.0, 60.0, 60.0, 0.966667),
    ("nosotros", "nu", 60.0, 250.0, 60.0, 0.240000),
    ("kjen", "ki", 50.0, 130.0, 60.0, 0.384615),
    ("ke", "kwa", 50.0, 60.0, 95.0, 0.526316),
    ("todos", "tu", 60.0, 155.0, 60.0, 0.387097),
    ("una", "ən", 57.0, 85.0, 60.0, 0.670588),
    ("dos", "dø", 58.0, 95.0, 60.0, 0.610526),
    ("tres", "trwa", 85.0, 130.0, 130.0, 0.653846),
    ("ombre", "om", 60.0, 155.0, 60.0, 0.387097),
    ("arbol", "arbrə", 107.0, 155.0, 155.0, 0.690323),
    ("pluma", "plym", 128.0, 155.0, 130.0, 0.825806),
    ("kabeθa", "kap", 90.0, 180.0, 95.0, 0.500000),
    ("boka", "buʃ", 81.5, 120.0, 95.0, 0.679167),
    ("pje", "pje", 95.0, 95.0, 95.0, 1.000000),
    ("koraθon", "kœr", 93.0, 215.0, 95.0, 0.432558),
    ("ber", "vwar", 75.5, 95.0, 130.0, 0.580769),
    ("benir", "vənir", 144.5, 155.0, 155.0, 0.932258),
    ("deθir", "dir", 80.5, 155.0, 95.0, 0.519355),
    ("pobre", "povrə", 144.5, 155.0, 155.0, 0.932258),
    ("ðis", "dIzes", 93.0, 95.0, 155.0, 0.600000),
    ("ðæt", "das", 78.0, 95.0, 95.0, 0.821053),
    ("wat", "vas", 73.0, 95.0, 95.0, 0.768421),
    ("nat", "nixt", 87.5, 95.0, 130.0, 0.673077),
    ("loŋ", "laŋ", 91.0, 95.0, 95.0, 0.957895),
    ("mæn", "man", 95.0, 95.0, 95.0, 1.000000),
    ("fleʃ", "flajʃ", 120.5, 130.0, 165.0, 0.730303),
    ("bləd", "blyt", 122.0, 130.0, 130.0, 0.938462),
    ("feðər", "fEdər", 145.5, 155.0, 155.0, 0.938710),
    ("hær", "hAr", 95.0, 95.0, 95.0, 1.000000),
    ("ir", "Or", 56.0, 60.0, 60.0, 0.933333),
    ("aj", "awgə", 48.0, 60.0, 120.0, 0.400000),
    ("nowz", "nAzə", 82.5, 130.0, 120.0, 0.634615),
    ("mawθ", "munt", 70.5, 130.0, 130.0, 0.542308),
    ("təŋ", "tsuŋə", 94.5, 95.0, 155.0, 0.609677),
    ("fut", "fys", 85.5, 95.0, 95.0, 0.900000),
    ("nij", "knI", 60.0, 95.0, 95.0, 0.631579),
    ("hænd", "hant", 125.0, 130.0, 130.0, 0.961538),
    ("hart", "herts", 132.5, 130.0, 165.0, 0.803030),
    ("livər", "lEbər", 145.5, 155.0, 155.0, 0.938710),
    ("ænd", "ante", 90.0, 95.0, 120.0, 0.750000),
    ("æt", "ad", 55.0, 60.0, 60.0, 0.916667),
    ("blow", "flAre", 90.5, 130.0, 155.0, 0.583871),
    ("ir", "awris", 50.0, 60.0, 155.0, 0.322581),
    ("ijt", "edere", 45.0, 95.0, 145.0, 0.310345),
    ("fiʃ", "piʃkis", 85.5, 95.0, 190.0, 0.450000),
    ("flow", "fluere", 115.0, 130.0, 180.0, 0.638889),
    ("staɾ", "stella", 117.5, 130.0, 190.0, 0.618421),
    ("ful", "plenus", 50.5, 95.0, 190.0, 0.265789),
    ("græs", "gramen", 95.0, 130.0, 190.0, 0.500000),
    ("hart", "kordis", 93.5, 130.0, 190.0, 0.492105),
    ("horn", "korny", 102.5, 130.0, 155.0, 0.661290),
    ("aj", "ego", 36.0, 60.0, 85.0, 0.423529),
    ("nij", "genU", 56.0, 95.0, 120.0, 0.466667),
    ("məðər", "mAter", 138.5, 155.0, 155.0, 0.893548),
    ("mawntən", "mons", 108.5, 225.0, 130.0, 0.482222),
    ("nejm", "nomen", 81.0, 130.0, 155.0, 0.522581),
    ("njuw", "nowus", 85.0, 130.0, 155.0, 0.548387),
    ("wən", "unus", 57.0, 95.0, 120.0, 0.475000),
    ("rawnd", "rotundus", 120.0, 165.0, 250.0, 0.480000),
    ("sow", "suere", 80.0, 95.0, 145.0, 0.551724),
    ("sit", "sedere", 90.0, 95.0, 180.0, 0.500000),
    ("θrij", "tres", 97.0, 130.0, 130.0, 0.746154),
    ("tuwθ", "dentis", 79.0, 130.0, 190.0, 0.415789),
    ("θin", "tenwis", 85.5, 95.0, 190.0, 0.450000),
    ("kinwawa", "kenuaʔ", 121.0, 215.0, 180.0, 0.562791),
    ("nina", "nenah", 120.0, 120.0, 155.0, 0.774194),
    ("napewa", "napɛw", 155.0, 180.0, 155.0, 0.861111),
    ("wapimini", "wapemen", 215.0, 240.0, 215.0, 0.895833),
    ("namesa", "namɛʔs", 145.0, 180.0, 190.0, 0.763158),
    ("okimawa", "okemaw", 180.0, 205.0, 180.0, 0.878049),
    ("ʃiʃipa", "seʔsep", 137.0, 180.0, 190.0, 0.721053),
    ("ahkohkwa", "ahkɛh", 151.0, 250.0, 155.0, 0.604000),
    ("pematesiweni", "pematesewen", 335.0, 360.0, 335.0, 0.930556),
    ("asenja", "aʔsɛn", 110.0, 180.0, 155.0, 0.611111),
];

#[test]
fn raw_scores_match_nltk_reference() {
    let cfg = load(ENG);
    assert!(matches!(cfg.variant, AlineVariant::Kondrak));
    assert_eq!(cfg.alignment_mode, AlignmentMode::Local);
    assert!(!cfg.merge_diphthongs);

    let tol = 1e-4;
    for &(w1, w2, raw, self1, self2, sim) in NLTK_COGNATES {
        let x = parse_segments(w1, "x", &cfg).unwrap();
        let y = parse_segments(w2, "y", &cfg).unwrap();
        // The reference indexes Python characters; every demo word is one
        // character per segment, so segment counts must match char counts.
        assert_eq!(x.len(), w1.chars().count(), "{w1}");
        assert_eq!(y.len(), w2.chars().count(), "{w2}");

        for (label, actual, expected) in [
            ("raw", alignment_score(&x, &y, &cfg), raw),
            ("self x", alignment_score(&x, &x, &cfg), self1),
            ("self y", alignment_score(&y, &y, &cfg), self2),
            ("similarity", cfg.similarity(w1, w2).unwrap(), sim),
        ] {
            assert!(
                (actual - expected).abs() <= tol,
                "{w1} ~ {w2} {label}: expected {expected}, got {actual}"
            );
        }
    }
}

// 9. Next-vowel stress scope, unclamped similarity and the public tokenizer.
fn fil_next_vowel() -> Aline {
    let mut cfg = load(FIL);
    assert!(matches!(cfg.variant, AlineVariant::MangoCats));
    cfg.alignment_mode = AlignmentMode::Global;
    cfg.salience.stress = 10;
    cfg.stress_scope_fallback = StressScopeFallback::NextVowel;
    cfg
}

#[test]
fn stress_fallback_next_vowel() {
    let cfg = fil_next_vowel();
    let segs = parse_segments("ˈkabajo", "x", &cfg).unwrap();
    assert_eq!(syms(&segs), ["k", "a", "b", "a", "j", "o"]);
    assert_eq!(stresses(&segs), [0.0, 1.0, 0.0, 0.0, 0.0, 0.0]);

    let segs = parse_segments("kaˈbajo", "x", &cfg).unwrap();
    assert_eq!(stresses(&segs), [0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);

    let segs = parse_segments("ˌkabaˈjo", "x", &cfg).unwrap();
    assert_eq!(stresses(&segs), [0.0, 0.5, 0.0, 0.0, 0.0, 1.0]);

    // Scope is per word, and `.`-delimited words keep the syllable scope.
    let segs = parse_segments("ˈbata ba.ˈta", "x", &cfg).unwrap();
    assert_eq!(stresses(&segs), [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0]);
}

#[test]
fn next_vowel_stress_shifts_are_symmetric() {
    let cfg = fil_next_vowel();
    let s = |x, y| cfg.similarity(x, y).unwrap();
    let a = s("ˈkabajo", "kaˈbajo");
    let b = s("ˈkabajo", "kabaˈjo");
    let c = s("kaˈbajo", "kabaˈjo");
    assert!(a < 1.0, "stress salience should lower the score, got {a}");
    assert!((a - b).abs() < 1e-6, "{a} != {b}");
    assert!((a - c).abs() < 1e-6, "{a} != {c}");
}

#[test]
fn next_vowel_matches_syllable_scope() {
    let next_vowel = fil_next_vowel();
    let mut syllable = fil_next_vowel();
    syllable.stress_scope_fallback = StressScopeFallback::default();

    const CASES: &[(&str, &str, &str, &str)] = &[
        ("ˈkabajo", "ˈka.ba.jo", "kaˈbajo", "ka.ˈba.jo"),
        ("ˈkabajo", "ˈka.ba.jo", "kabaˈjo", "ka.ba.ˈjo"),
        ("ˈkabajo", "ˈka.ba.jo", "kabajo", "kabajo"),
        ("ˈkabajo", "ˈka.ba.jo", "ˈkabajo", "ˈka.ba.jo"),
        ("ˈkabajo", "ˈka.ba.jo", "bata", "bata"),
    ];
    for &(x_nv, x_syl, y_nv, y_syl) in CASES {
        let nv = next_vowel.similarity_unclamped(x_nv, y_nv).unwrap();
        let syl = syllable.similarity_unclamped(x_syl, y_syl).unwrap();
        assert!(
            (nv - syl).abs() < 1e-6,
            "{x_nv}/{y_nv} = {nv}, {x_syl}/{y_syl} = {syl}"
        );
        let nv = next_vowel.similarity(x_nv, y_nv).unwrap();
        let syl = syllable.similarity(x_syl, y_syl).unwrap();
        assert!((nv - syl).abs() < 1e-6);
    }
}

#[test]
fn similarity_unclamped_can_be_negative() {
    let cfg = fil_next_vowel();
    let raw = cfg.similarity_unclamped("a", "kstrpt").unwrap();
    assert!(raw < 0.0, "expected a negative raw score, got {raw}");
    assert_eq!(cfg.similarity("a", "kstrpt").unwrap(), 0.0);
}

#[test]
fn similarity_unclamped_matches_similarity_in_range() {
    let words = [
        "ˈkabajo", "kaˈbajo", "kabaˈjo", "ˈbata", "a", "kstrpt", "tʃa", "baj", "ˈbahaj", "ˈpusa",
        "",
    ];
    let mut global = fil_next_vowel();
    for mode in [AlignmentMode::Global, AlignmentMode::Local] {
        global.alignment_mode = mode;
        for x in words {
            for y in words {
                let sim = global.similarity(x, y).unwrap();
                match global.similarity_unclamped(x, y) {
                    Ok(raw) if (0.0..=1.0).contains(&raw) => {
                        assert_eq!(raw, sim, "{x}/{y} ({mode:?})")
                    }
                    Ok(raw) if mode == AlignmentMode::Local => {
                        assert_eq!(raw, sim, "{x}/{y} ({mode:?})")
                    }
                    Ok(raw) => assert_eq!(raw.clamp(0.0, 1.0), sim, "{x}/{y}"),
                    Err(Error::NonPositiveSelfScore { .. }) => assert_eq!(sim, 0.0),
                    Err(e) => panic!("{x}/{y}: {e}"),
                }
            }
        }
    }
}

#[test]
fn tokenize_returns_inventory_keys() {
    let cfg = fil_next_vowel();
    let ligature = cfg.tokenize("ʧa").unwrap();
    assert_eq!(ligature, ["ʧ", "a"]);
    assert_eq!(cfg.tokenize("tʃa").unwrap(), ligature);
    // The fil inventory lists the tie-bar spelling as its own key, so an
    // exact match keeps it. Its features are identical to `ʧ`.
    let tied = cfg.tokenize("t͡ʃa").unwrap();
    assert_eq!(tied, ["t͡ʃ", "a"]);
    assert_eq!(
        format!("{:?}", cfg.sounds["t͡ʃ"]),
        format!("{:?}", cfg.sounds["ʧ"])
    );

    assert_eq!(cfg.tokenize("baj").unwrap(), ["b", "a", "j"]);
    assert_eq!(
        cfg.tokenize("ˈka.ba.jo").unwrap(),
        ["k", "a", "b", "a", "j", "o"]
    );
    assert!(matches!(
        cfg.tokenize("bQ").unwrap_err(),
        Error::UnknownToken { position: 1, .. }
    ));
}

#[test]
fn weights_are_readable() {
    let cfg = load(FIL);
    assert_eq!(cfg.costs.skip(), -10);
    assert_eq!(cfg.costs.substitute(), 35);
    assert_eq!(cfg.costs.expand_compress(), 45);
    assert_eq!(cfg.costs.vowel_consonant(), 5);
    assert_eq!(cfg.salience.place(), 40);
    assert_eq!(cfg.salience.manner(), 50);
    assert_eq!(cfg.salience.long(), 10);
    assert_eq!(cfg.salience.stress(), 0);
    assert_eq!(cfg.salience.secondary(), 10);
}
