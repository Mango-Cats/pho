//! aline
//!
//! A Rust implementation of the ALINE phonetic similarity algorithm.
//!
//! This ports the core dynamic-programming scoring logic from Kondrak (2002).
//!
//! ## What ALINE computes
//!
//! - `alignment_score(a, b)` computes the raw optimal alignment score
//!   between two phonetic segment sequences (local by default, see
//!   `alignment_mode`).
//! - `similarity(a, b)` computes a normalized score in $[0, 1]$:
//!   $$\text{similarity}(a,b) = \frac{\text{alignment\_score}(a,b)}{\max(\text{alignment\_score}(a,a),\,\text{alignment\_score}(b,b))}$$
//!
//! Local alignment allows the dynamic programming to restart at 0 (Smith-Waterman style),
//! so best-matching subsequences dominate the score. Global alignment charges leading and
//! trailing mismatches as indels, clamping the normalized score to $[0, 1]$.
//!
//! `similarity_unclamped(a, b)` returns the same ratio without clamping in
//! either mode, and an `Error::NonPositiveSelfScore` where `similarity`
//! returns 0.
//!
//! The loaded weights can be read through `config.costs` and
//! `config.salience` getters, such as `config.salience.stress()`.
//!
//! ## Segments and Tokenization
//!
//! Inputs are split into grapheme clusters and matched against phonetic
//! inventory keys longest-first. Spellings are flexible: tie bars are optional,
//! and standard affricate ligatures or rhotic vowels automatically match their
//! decomposed forms. Multi-grapheme diphthongs are only merged when
//! `merge_diphthongs = true`. Stress, length, and syllable marks are handled
//! during parsing and do not become standalone segments. `tokenize(ipa)`
//! returns the inventory key of each segment, exactly as the scorer sees them.
//!
//! ## Stress scope (MangoCats)
//!
//! A stress mark covers its `.`-delimited syllable. In a word with no `.`
//! breaks, `stress_scope_fallback` decides: `"error"` (default) rejects the
//! input, `"until_next_mark"` stresses every following segment until the next
//! mark, and `"next_vowel"` stresses only the first vowel after the mark.
//!
//! ## Example
//!
//! ```rust
//! use pho::{algorithms::{Aline, Algorithm}, utils::io::import};
//!
//! let algo: Aline = import("algorithm_configs/eng/aline.toml").unwrap();
//! let score = algo.similarity("s", "s").unwrap();
//! assert!((score - 1.0).abs() < 1e-6);
//! ```

mod alignment;
pub mod config;
#[cfg(test)]
mod regression_tests;
mod scoring;
use crate::algorithms::Algorithm;
use crate::error::{Error, Result};
use config::{AlignmentMode, Aline, AlineVariant, Binary, PhoneticFeatures, StressScopeFallback};
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

/// One phonetic segment of a parsed IPA string.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Segment {
    /// Inventory key (a key of `Aline::sounds`).
    pub(crate) sym: String,
    /// Stress weight: 0.0 for unstressed, 0.5 for secondary, 1.0 for primary.
    pub(crate) stress: f32,
    /// Length override from a following full or half-long length mark.
    /// `None` means use the inventory default. Only scored for vowels.
    pub(crate) long: Option<f32>,
}

/// A classified grapheme cluster of the input.
enum Token<'a> {
    /// Part of a segment, where consecutive graphemes match greedily.
    Grapheme(&'a str),
    /// Stress level: 1.0 for primary or 0.5 for secondary.
    Stress(f32),
    /// Syllable boundary marker ('.').
    SyllableBreak,
    /// Whitespace or end of input.
    WordBreak,
    /// Length mark, modifying the previous segment.
    Length {
        half: bool,
    },
    Ignored,
}

fn classify(g: &str) -> Token<'_> {
    match g {
        "ˈ" => Token::Stress(1.0),
        "ˌ" => Token::Stress(0.5),
        "." => Token::SyllableBreak,
        "ː" => Token::Length { half: false },
        "ˑ" => Token::Length { half: true },
        _ if g.chars().all(char::is_whitespace) => Token::WordBreak,
        _ if g.chars().all(|c| {
            c.is_numeric() || "[]/\\,;:()|{}<>\"'-+_&".contains(c) || "‖‿⁻".contains(c)
        }) =>
        {
            Token::Ignored
        }
        _ => Token::Grapheme(g),
    }
}

/// Tie bars joining two letters into one segment.
const TIE_BARS: [char; 2] = ['\u{0361}', '\u{035C}'];

/// Single-codepoint IPA symbols that are equivalent to a multi-character
/// spelling: the affricate and other digraph ligatures, the rhotic-hook
/// vowels and the velarized l.
const MULTI_CHAR_SYMBOLS: [(char, &str); 14] = [
    ('ʣ', "dz"),
    ('ʤ', "dʒ"),
    ('ʥ', "dʑ"),
    ('ꭦ', "dʐ"),
    ('ʦ', "ts"),
    ('ʧ', "tʃ"),
    ('ʨ', "tɕ"),
    ('ꭧ', "tʂ"),
    ('ʩ', "fŋ"),
    ('ʪ', "ls"),
    ('ʫ', "lz"),
    ('ɚ', "ə˞"),
    ('ɝ', "ɜ˞"),
    ('ɫ', "l\u{0334}"),
];

/// Converts an IPA string into canonical form by stripping tie bars and expanding ligatures.
fn canonical(ipa: &str) -> String {
    let mut out = String::with_capacity(ipa.len());
    for c in ipa.chars().filter(|c| !TIE_BARS.contains(c)) {
        match MULTI_CHAR_SYMBOLS.iter().find(|(lig, _)| *lig == c) {
            Some((_, expanded)) => out.push_str(expanded),
            None => out.push(c),
        }
    }
    out
}

/// Inventory lookup that accepts any spelling of a symbol.
struct Inventory<'a> {
    config: &'a Aline,
    /// Canonical form to inventory key mapping. When several keys share a canonical
    /// form, the shortest key wins.
    by_canonical: HashMap<String, &'a str>,
    /// Longest key, in graphemes, over both spellings.
    max_len: usize,
}

impl<'a> Inventory<'a> {
    fn new(config: &'a Aline) -> Self {
        let mut keys: Vec<&str> = config.sounds.keys().map(String::as_str).collect();
        keys.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));

        let mut by_canonical = HashMap::new();
        let mut max_len = 1;
        for key in keys {
            let canon = canonical(key);
            max_len = max_len
                .max(key.graphemes(true).count())
                .max(canon.graphemes(true).count());
            by_canonical.entry(canon).or_insert(key);
        }
        Self {
            config,
            by_canonical,
            max_len,
        }
    }

    /// The inventory key for `ipa`, preferring an exact match.
    fn resolve(&self, ipa: &str) -> Option<(&'a str, &'a PhoneticFeatures)> {
        let key = match self.config.sounds.get_key_value(ipa) {
            Some((key, _)) => key.as_str(),
            None => *self.by_canonical.get(&canonical(ipa))?,
        };
        Some((key, &self.config.sounds[key]))
    }
}

/// Matches a run of graphemes against the sound inventory using the longest match.
///
/// Spans are compared by canonical form so ligatures and decomposed forms are interchangeable.
/// Graphemes that match no key are emitted directly so validation can report them.
fn match_run(run: &[&str], stress: f32, inventory: &Inventory, out: &mut Vec<Segment>) {
    let mut i = 0;
    while i < run.len() {
        let mut taken = 1;
        let mut sym = run[i].to_string();
        for n in (1..=inventory.max_len.min(run.len() - i)).rev() {
            if let Some((key, sound)) = inventory.resolve(&run[i..i + n].concat())
                && (inventory.config.merge_diphthongs
                    || sound.is_consonant()
                    || key.graphemes(true).nth(1).is_none())
            {
                taken = n;
                sym = key.to_string();
                break;
            }
        }
        out.push(Segment {
            sym,
            stress,
            long: None,
        });
        i += taken;
    }
}

/// Parses an IPA string into segments.
///
/// Graphemes are matched greedily against the inventory. Length marks update the previous
/// segment, while syllable and word boundaries reset state.
/// In Kondrak mode, stress marks are ignored. In MangoCats mode, stress marks set the stress
/// level for following segments until the next syllable or stress marker. In a word with no
/// syllable marks, [`StressScopeFallback`] decides the scope instead.
fn parse_segments(ipa: &str, input_name: &'static str, config: &Aline) -> Result<Vec<Segment>> {
    let use_stress = matches!(config.variant, AlineVariant::MangoCats);
    let inventory = Inventory::new(config);
    let plus = config.values.binary[Binary::Plus];
    let minus = config.values.binary[Binary::Minus];

    let mut segments = Vec::new();
    let mut run: Vec<&str> = Vec::new();
    let mut stress = 0.0f32;
    let mut word_has_mark = false;
    let mut word_has_break = false;
    // Index of the word's first segment, and each stress mark of the word as
    // (index of the next segment, level), for the `NextVowel` fallback.
    let mut word_start = 0;
    let mut word_marks: Vec<(usize, f32)> = Vec::new();

    let tokens = ipa
        .graphemes(true)
        .map(classify)
        .chain(std::iter::once(Token::WordBreak));

    for token in tokens {
        if let Token::Grapheme(g) = token {
            run.push(g);
            continue;
        }

        let follows_segment = !run.is_empty();
        match_run(&run, stress, &inventory, &mut segments);
        run.clear();

        match token {
            Token::Grapheme(_) | Token::Ignored => {}
            Token::Stress(level) => {
                if use_stress {
                    stress = level;
                    word_has_mark = true;
                    word_marks.push((segments.len(), level));
                }
            }
            Token::SyllableBreak => {
                stress = 0.0;
                word_has_break = true;
            }
            Token::WordBreak => {
                if word_has_mark
                    && !word_has_break
                    && config.stress_scope_fallback == StressScopeFallback::Error
                {
                    return Err(Error::AmbiguousStressScope { input_name });
                }
                if word_has_mark
                    && !word_has_break
                    && config.stress_scope_fallback == StressScopeFallback::NextVowel
                {
                    restress_next_vowel(
                        &mut segments[word_start..],
                        &word_marks,
                        word_start,
                        config,
                    );
                }
                stress = 0.0;
                word_has_mark = false;
                word_has_break = false;
                word_start = segments.len();
                word_marks.clear();
            }
            Token::Length { half } => {
                if follows_segment && let Some(last) = segments.last_mut() {
                    last.long = Some(if half { (plus + minus) / 2.0 } else { plus });
                }
            }
        }
    }
    Ok(segments)
}

/// Applies the `NextVowel` stress scope to one word: each mark stresses only the first vowel
/// segment at or after its position, and every other segment is unstressed.
///
/// `marks` holds (absolute segment index, level) pairs and `offset` is the absolute index of
/// `word[0]`.
fn restress_next_vowel(
    word: &mut [Segment],
    marks: &[(usize, f32)],
    offset: usize,
    config: &Aline,
) {
    for seg in word.iter_mut() {
        seg.stress = 0.0;
    }
    for &(pos, level) in marks {
        let is_vowel = |seg: &&mut Segment| {
            config
                .sounds
                .get(seg.sym.as_str())
                .is_some_and(PhoneticFeatures::is_vowel)
        };
        if let Some(seg) = word[pos - offset..].iter_mut().find(is_vowel) {
            seg.stress = level;
        }
    }
}

fn validate_segments(segments: &[Segment], input_name: &'static str, config: &Aline) -> Result<()> {
    for (pos, seg) in segments.iter().enumerate() {
        if !config.sounds.contains_key(seg.sym.as_str()) {
            return Err(Error::UnknownToken {
                token: seg.sym.clone(),
                position: pos,
                input_name,
                context: "ALINE config sound inventory",
            });
        }
    }
    Ok(())
}

impl Algorithm for Aline {
    fn requires_transcription(&self) -> bool {
        true
    }

    /// Normalized similarity: `score / max(self_x, self_y)`.
    ///
    /// In global alignment mode the raw score can be negative, so the result
    /// is clamped to `[0, 1]`. Local mode is returned unclamped. Returns 0 if
    /// both self-alignment scores are non-positive. See
    /// [`Aline::similarity_unclamped`] for the raw ratio.
    fn similarity(&self, x: &str, y: &str) -> Result<f32> {
        let sim = match self.similarity_unclamped(x, y) {
            Ok(sim) => sim,
            Err(Error::NonPositiveSelfScore { .. }) => return Ok(0.0),
            Err(e) => return Err(e),
        };
        Ok(match self.alignment_mode {
            AlignmentMode::Local => sim,
            AlignmentMode::Global => sim.clamp(0.0, 1.0),
        })
    }
}

impl Aline {
    /// Normalized similarity `score / max(self_x, self_y)` without clamping,
    /// in either alignment mode. In global mode the result can be negative.
    ///
    /// Returns [`Error::NonPositiveSelfScore`] if both self-alignment scores
    /// are non-positive, where [`Algorithm::similarity`] returns 0.
    pub fn similarity_unclamped(&self, x: &str, y: &str) -> Result<f32> {
        use alignment::alignment_score;

        let x_segs = parse_segments(x, "x", self)?;
        let y_segs = parse_segments(y, "y", self)?;

        validate_segments(&x_segs, "x", self)?;
        validate_segments(&y_segs, "y", self)?;

        let score = alignment_score(&x_segs, &y_segs, self);
        let x_self = alignment_score(&x_segs, &x_segs, self);
        let y_self = alignment_score(&y_segs, &y_segs, self);

        let denom = x_self.max(y_self);
        if denom <= 0.0 {
            return Err(Error::NonPositiveSelfScore { denominator: denom });
        }
        Ok(score / denom)
    }

    /// Splits an IPA string into segments exactly as the scorer sees them and
    /// returns the inventory key of each one.
    ///
    /// Stress, length and syllable marks do not become segments. Returns
    /// [`Error::UnknownToken`] for a symbol that is not in the inventory.
    pub fn tokenize(&self, ipa: &str) -> Result<Vec<String>> {
        let segments = parse_segments(ipa, "ipa", self)?;
        validate_segments(&segments, "ipa", self)?;
        Ok(segments.into_iter().map(|seg| seg.sym).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::Algorithm;
    use crate::{
        algorithms::{
            Aline,
            aline::config::{Back, Binary, High, Manner, PhoneticFeatures, Place},
        },
        error::Result,
        utils::io::import,
    };

    const TOML_PATH: &str = "algorithm_configs/eng/aline.toml";

    fn load() -> Aline {
        match import(TOML_PATH) {
            Ok(config) => config,
            Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
        }
    }

    #[test]
    fn costs_skip() {
        assert_eq!(load().costs.skip, -10);
    }

    #[test]
    fn costs_substitute() {
        assert_eq!(load().costs.substitute, 35);
    }

    #[test]
    fn costs_expand_compress() {
        assert_eq!(load().costs.expand_compress, 45);
    }

    #[test]
    fn costs_vowel_consonant() {
        assert_eq!(load().costs.vowel_consonant, 5);
    }

    #[test]
    fn epsilon_parses() {
        let epsilon = load().epsilon;
        assert!(
            (epsilon - 0.001).abs() < 1e-6,
            "expected epsilon≈0.001, got {epsilon}"
        );
    }

    #[test]
    fn salience_place() {
        assert_eq!(load().salience.place, 40);
    }

    #[test]
    fn salience_manner() {
        assert_eq!(load().salience.manner, 50);
    }

    #[test]
    fn salience_nasal() {
        assert_eq!(load().salience.nasal, 20);
    }

    #[test]
    fn salience_voice() {
        assert_eq!(load().salience.voice, 5);
    }

    #[test]
    fn salience_retroflex() {
        assert_eq!(load().salience.retroflex, 10);
    }

    #[test]
    fn salience_lateral() {
        assert_eq!(load().salience.lateral, 10);
    }

    #[test]
    fn salience_aspirated() {
        assert_eq!(load().salience.aspirated, 5);
    }

    #[test]
    fn salience_syllabic() {
        assert_eq!(load().salience.syllabic, 5);
    }

    #[test]
    fn salience_long() {
        assert_eq!(load().salience.long, 0);
    }

    #[test]
    fn salience_high() {
        assert_eq!(load().salience.high, 3);
    }

    #[test]
    fn salience_back() {
        assert_eq!(load().salience.back, 2);
    }

    #[test]
    fn salience_round() {
        assert_eq!(load().salience.round, 2);
    }

    #[test]
    fn place_values_bilabial() {
        assert_eq!(load().values.place[Place::Bilabial], 1.0);
    }

    #[test]
    fn place_values_alveolar() {
        assert_eq!(load().values.place[Place::Alveolar], 0.85);
    }

    #[test]
    fn place_values_glottal() {
        assert_eq!(load().values.place[Place::Glottal], 0.1);
    }

    #[test]
    fn place_values_vowel() {
        assert_eq!(load().values.place[Place::Vowel], -1.0);
    }

    #[test]
    fn manner_values_stop() {
        assert_eq!(load().values.manner[Manner::Stop], 1.0);
    }

    #[test]
    fn manner_values_fricative() {
        assert_eq!(load().values.manner[Manner::Fricative], 0.85);
    }

    #[test]
    fn manner_values_approximant() {
        assert_eq!(load().values.manner[Manner::Approximant], 0.6);
    }

    #[test]
    fn manner_values_low_vowel() {
        assert_eq!(load().values.manner[Manner::LowVowel], 0.0);
    }

    #[test]
    fn height_values_high() {
        assert_eq!(load().values.high[High::High], 1.0);
    }

    #[test]
    fn height_values_mid() {
        assert_eq!(load().values.high[High::Mid], 0.5);
    }

    #[test]
    fn height_values_low() {
        assert_eq!(load().values.high[High::Low], 0.0);
    }

    #[test]
    fn backness_values_front() {
        assert_eq!(load().values.back[Back::Front], 1.0);
    }

    #[test]
    fn backness_values_central() {
        assert_eq!(load().values.back[Back::Central], 0.5);
    }

    #[test]
    fn backness_values_back() {
        assert_eq!(load().values.back[Back::Back], 0.0);
    }

    #[test]
    fn binary_values_plus() {
        assert_eq!(load().values.binary[Binary::Plus], 1.0);
    }

    #[test]
    fn binary_values_minus() {
        assert_eq!(load().values.binary[Binary::Minus], 0.0);
    }

    #[test]
    fn sounds_contains_s() {
        assert!(load().sounds.contains_key("s"), "expected sound 's' in map");
    }

    #[test]
    fn sounds_contains_b() {
        assert!(load().sounds.contains_key("b"), "expected sound 'b' in map");
    }

    #[test]
    fn sounds_contains_a() {
        assert!(load().sounds.contains_key("a"), "expected sound 'a' in map");
    }

    #[test]
    fn sounds_contains_i() {
        assert!(load().sounds.contains_key("i"), "expected sound 'i' in map");
    }

    #[test]
    fn sounds_count() {
        assert!(
            load().sounds.len() >= 80,
            "expected a reasonably complete IPA inventory"
        );
    }

    #[test]
    fn sounds_contains_regression_symbols() {
        let config = load();
        for sym in [
            "ə", "ð", "θ", "ʃ", "ŋ", "ɲ", "ɾ", "ʔ", "ø", "œ", "A", "E", "I", "O", "U", "e̞", "ø̞",
        ] {
            assert!(
                config.sounds.contains_key(sym),
                "expected sound '{sym}' in map"
            );
        }
    }

    #[test]
    fn sound_s_is_consonant() {
        assert!(matches!(load().sounds["s"], PhoneticFeatures::Consonant(_)));
    }

    #[test]
    fn sound_s_place() {
        let config = load();
        let PhoneticFeatures::Consonant(c) = &config.sounds["s"] else {
            panic!("'s' is not a consonant");
        };
        assert!(matches!(c.common.place, Place::Alveolar));
    }

    #[test]
    fn sound_s_manner() {
        let config = load();
        let PhoneticFeatures::Consonant(c) = &config.sounds["s"] else {
            panic!("'s' is not a consonant");
        };
        assert!(matches!(c.common.manner, Manner::Fricative));
    }

    #[test]
    fn sound_s_voice_is_minus() {
        let config = load();
        let PhoneticFeatures::Consonant(c) = &config.sounds["s"] else {
            panic!("'s' is not a consonant");
        };
        assert!(matches!(c.common.voice, Binary::Minus));
    }

    #[test]
    fn sound_s_nasal_is_minus() {
        let config = load();
        let PhoneticFeatures::Consonant(c) = &config.sounds["s"] else {
            panic!("'s' is not a consonant");
        };
        assert!(matches!(c.common.nasal, Binary::Minus));
    }

    #[test]
    fn sound_s_lateral_is_minus() {
        let config = load();
        let PhoneticFeatures::Consonant(c) = &config.sounds["s"] else {
            panic!("'s' is not a consonant");
        };
        assert!(matches!(c.common.lateral, Binary::Minus));
    }

    #[test]
    fn sound_b_is_consonant() {
        assert!(matches!(load().sounds["b"], PhoneticFeatures::Consonant(_)));
    }

    #[test]
    fn sound_b_place() {
        let config = load();
        let PhoneticFeatures::Consonant(c) = &config.sounds["b"] else {
            panic!("'b' is not a consonant");
        };
        assert!(matches!(c.common.place, Place::Bilabial));
    }

    #[test]
    fn sound_b_manner() {
        let config = load();
        let PhoneticFeatures::Consonant(c) = &config.sounds["b"] else {
            panic!("'b' is not a consonant");
        };
        assert!(matches!(c.common.manner, Manner::Stop));
    }

    #[test]
    fn sound_b_voice_is_plus() {
        let config = load();
        let PhoneticFeatures::Consonant(c) = &config.sounds["b"] else {
            panic!("'b' is not a consonant");
        };
        assert!(matches!(c.common.voice, Binary::Plus));
    }

    #[test]
    fn sound_a_is_vowel() {
        assert!(matches!(load().sounds["a"], PhoneticFeatures::Vowel(_)));
    }

    #[test]
    fn sound_a_high_is_low() {
        let config = load();
        let PhoneticFeatures::Vowel(v) = &config.sounds["a"] else {
            panic!("'a' is not a vowel");
        };
        assert!(matches!(v.high, High::Low));
    }

    #[test]
    fn sound_a_back_is_front() {
        let config = load();
        let PhoneticFeatures::Vowel(v) = &config.sounds["a"] else {
            panic!("'a' is not a vowel");
        };
        assert!(matches!(v.back, Back::Front));
    }

    #[test]
    fn sound_a_round_is_minus() {
        let config = load();
        let PhoneticFeatures::Vowel(v) = &config.sounds["a"] else {
            panic!("'a' is not a vowel");
        };
        assert!(matches!(v.round, Binary::Minus));
    }

    #[test]
    fn sound_a_syllabic_is_plus() {
        let config = load();
        let PhoneticFeatures::Vowel(v) = &config.sounds["a"] else {
            panic!("'a' is not a vowel");
        };
        assert!(matches!(v.common.syllabic, Binary::Plus));
    }

    #[test]
    fn sound_a_long_is_minus() {
        let config = load();
        let PhoneticFeatures::Vowel(v) = &config.sounds["a"] else {
            panic!("'a' is not a vowel");
        };
        assert!(matches!(v.long, Binary::Minus));
    }

    #[test]
    fn sound_i_is_vowel() {
        assert!(matches!(load().sounds["i"], PhoneticFeatures::Vowel(_)));
    }

    #[test]
    fn sound_i_high_is_high() {
        let config = load();
        let PhoneticFeatures::Vowel(v) = &config.sounds["i"] else {
            panic!("'i' is not a vowel");
        };
        assert!(matches!(v.high, High::High));
    }

    #[test]
    fn sound_i_back_is_front() {
        let config = load();
        let PhoneticFeatures::Vowel(v) = &config.sounds["i"] else {
            panic!("'i' is not a vowel");
        };
        assert!(matches!(v.back, Back::Front));
    }

    #[test]
    fn sound_i_round_is_minus() {
        let config = load();
        let PhoneticFeatures::Vowel(v) = &config.sounds["i"] else {
            panic!("'i' is not a vowel");
        };
        assert!(matches!(v.round, Binary::Minus));
    }

    #[test]
    fn rejects_non_toml_extension() {
        let result: Result<Aline> = import("notatoml.json");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_file() {
        let result: Result<Aline> = import("nonexistent.toml");
        assert!(result.is_err());
    }

    #[test]
    fn similarity_ignores_spaces() {
        let algo = load();
        let compact = algo.similarity("sa", "si").expect("compact input is valid");
        let spaced = algo
            .similarity(" s a ", " s i ")
            .expect("spaced input should be valid");

        assert!(
            (compact - spaced).abs() < 1e-6,
            "expected whitespace to be ignored, got compact={compact}, spaced={spaced}"
        );
    }

    #[test]
    fn similarity_ignores_superscript_minus() {
        let algo = load();
        let plain = algo.similarity("sa", "si").expect("plain input is valid");
        let marked = algo
            .similarity("sa⁻", "si⁻")
            .expect("U+207B should be ignored, not rejected as unknown");

        assert!(
            (plain - marked).abs() < 1e-6,
            "expected U+207B to be ignored, got plain={plain}, marked={marked}"
        );
    }
}
