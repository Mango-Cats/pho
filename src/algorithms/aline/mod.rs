//! aline
//!
//! A Rust implementation of the ALINE phonetic similarity algorithm.
//!
//! This ports the core dynamic-programming scoring logic from Kondrak (2002)
//! (and mirrors NLTK's reference implementation in `.dev/references/actual_aline.py`).
//!
//! ## What ALINE computes
//!
//! - `alignment_score(a, b)` computes the *raw* optimal **local** alignment score
//!   between two phonetic segment sequences.
//! - `similarity(a, b)` computes a normalized score in $[0, 1]$:
//!   $$\text{similarity}(a,b) = \frac{\text{alignment\_score}(a,b)}{\max(\text{alignment\_score}(a,a),\,\text{alignment\_score}(b,b))}$$
//!
//! Local alignment means the DP can restart at 0 (Smith–Waterman style), so the
//! best-matching subsequences dominate the score.
//!
//! ## Segments and Unicode
//!
//! IPA strings may contain multi-codepoint graphemes (e.g. letters with
//! combining diacritics). To avoid splitting these incorrectly, inputs are
//! tokenized into Unicode grapheme clusters.
//!
//! ## Example
//!
//! ```rust
//! use pho::{algorithms::{Aline, AlgorithmTrait}, utils::io::import};
//!
//! let algo: Aline = import("tests/config_sample_aline.toml").unwrap();
//! let score = algo.similarity("s", "s").unwrap();
//! assert!((score - 1.0).abs() < 1e-6);
//! ```

mod alignment;
pub mod config;
mod scoring;
use crate::{algorithms::AlgorithmTrait, errors::AlgorithmError};
use config::Aline;

impl AlgorithmTrait for Aline {
    fn similarity(&self, x: &str, y: &str) -> Result<f32, AlgorithmError> {
        use crate::utils::validate::validate_tokens;
        use alignment::alignment_score;

        use unicode_segmentation::UnicodeSegmentation;

        let x_valid = validate_tokens(
            UnicodeSegmentation::graphemes(x, true).map(str::to_string),
            "x",
            "ALINE config sound inventory",
            |segment| self.sounds.contains_key(segment),
        )?;

        let y_valid = validate_tokens(
            UnicodeSegmentation::graphemes(y, true).map(str::to_string),
            "y",
            "ALINE config sound inventory",
            |segment| self.sounds.contains_key(segment),
        )?;

        let score = alignment_score(&x_valid, &y_valid, self);

        let x_self = alignment_score(&x_valid, &x_valid, self);
        let y_self = alignment_score(&y_valid, &y_valid, self);
        let denom = x_self.max(y_self);

        if denom == 0.0 {
            return Ok(0.0);
        }

        Ok(score / denom)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        algorithms::{
            Aline,
            aline::config::{Back, Binary, High, Manner, PhoneticFeatures, Place},
        },
        utils::io::import,
    };

    const TOML_PATH: &str = "tests/config_sample_aline.toml";

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
        let result: Result<Aline, String> = import("notatoml.json");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "file must be a .toml");
    }

    #[test]
    fn rejects_missing_file() {
        let result: Result<Aline, String> = import("nonexistent.toml");
        assert!(result.is_err());
    }
}
