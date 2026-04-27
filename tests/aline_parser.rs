#[cfg(test)]
mod tests {
    use core::panic;

    use pho::{
        algorithms::aline::{
            config::{AlineConfig, RawAlineConfig},
            features::{Back, Binary, High, Manner, PhoneticFeatures, Place},
        },
        config::parse_toml_file,
    };

    // Path to the sample config — adjust if your test working directory differs.
    // `cargo test` runs from the crate root by default.
    const TOML_PATH: &str = "tests/aline_parser_data.toml";

    /// Unwrap the config or panic with a readable message.
    fn load() -> AlineConfig {
        let raw_config: Result<RawAlineConfig, String> = parse_toml_file(TOML_PATH);
        match raw_config {
            Ok(r) => r.into_config(),
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
        assert_eq!(load().values.place.bilabial, 1.0);
    }

    #[test]
    fn place_values_alveolar() {
        assert_eq!(load().values.place.alveolar, 0.85);
    }

    #[test]
    fn place_values_glottal() {
        assert_eq!(load().values.place.glottal, 0.1);
    }

    #[test]
    fn place_values_vowel() {
        assert_eq!(load().values.place.vowel, -1.0);
    }

    #[test]
    fn manner_values_stop() {
        assert_eq!(load().values.manner.stop, 1.0);
    }

    #[test]
    fn manner_values_fricative() {
        assert_eq!(load().values.manner.fricative, 0.85);
    }

    #[test]
    fn manner_values_approximant() {
        assert_eq!(load().values.manner.approximant, 0.6);
    }

    #[test]
    fn manner_values_low_vowel() {
        assert_eq!(load().values.manner.low_vowel, 0.0);
    }

    #[test]
    fn height_values_high() {
        assert_eq!(load().values.high.high, 1.0);
    }

    #[test]
    fn height_values_mid() {
        assert_eq!(load().values.high.mid, 0.5);
    }

    #[test]
    fn height_values_low() {
        assert_eq!(load().values.high.low, 0.0);
    }

    #[test]
    fn backness_values_front() {
        assert_eq!(load().values.back.front, 1.0);
    }

    #[test]
    fn backness_values_central() {
        assert_eq!(load().values.back.central, 0.5);
    }

    #[test]
    fn backness_values_back() {
        assert_eq!(load().values.back.back, 0.0);
    }

    #[test]
    fn binary_values_plus() {
        assert_eq!(load().values.binary.plus, 1.0);
    }

    #[test]
    fn binary_values_minus() {
        assert_eq!(load().values.binary.minus, 0.0);
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
        assert_eq!(load().sounds.len(), 4, "expected exactly 4 sounds");
    }

    #[test]
    fn sound_s_is_consonant() {
        let config = load();
        assert!(
            matches!(config.sounds["s"], PhoneticFeatures::Consonant(_)),
            "'s' should be a consonant"
        );
    }

    #[test]
    fn sound_s_place() {
        let config = load();
        if let PhoneticFeatures::Consonant(c) = &config.sounds["s"] {
            assert!(matches!(c.place, Place::Alveolar));
        } else {
            panic!("'s' is not a consonant");
        }
    }

    #[test]
    fn sound_s_manner() {
        let config = load();
        if let PhoneticFeatures::Consonant(c) = &config.sounds["s"] {
            assert!(matches!(c.manner, Manner::Fricative));
        } else {
            panic!("'s' is not a consonant");
        }
    }

    #[test]
    fn sound_s_voice_is_minus() {
        let config = load();
        if let PhoneticFeatures::Consonant(c) = &config.sounds["s"] {
            assert!(matches!(c.voice, Binary::Minus));
        } else {
            panic!("'s' is not a consonant");
        }
    }

    #[test]
    fn sound_s_nasal_is_minus() {
        let config = load();
        if let PhoneticFeatures::Consonant(c) = &config.sounds["s"] {
            assert!(matches!(c.nasal, Binary::Minus));
        } else {
            panic!("'s' is not a consonant");
        }
    }

    #[test]
    fn sound_s_lateral_is_minus() {
        let config = load();
        if let PhoneticFeatures::Consonant(c) = &config.sounds["s"] {
            assert!(matches!(c.lateral, Binary::Minus));
        } else {
            panic!("'s' is not a consonant");
        }
    }

    #[test]
    fn sound_b_is_consonant() {
        let config = load();
        assert!(
            matches!(config.sounds["b"], PhoneticFeatures::Consonant(_)),
            "'b' should be a consonant"
        );
    }

    #[test]
    fn sound_b_place() {
        let config = load();
        if let PhoneticFeatures::Consonant(c) = &config.sounds["b"] {
            assert!(matches!(c.place, Place::Bilabial));
        } else {
            panic!("'b' is not a consonant");
        }
    }

    #[test]
    fn sound_b_manner() {
        let config = load();
        if let PhoneticFeatures::Consonant(c) = &config.sounds["b"] {
            assert!(matches!(c.manner, Manner::Stop));
        } else {
            panic!("'b' is not a consonant");
        }
    }

    #[test]
    fn sound_b_voice_is_plus() {
        let config = load();
        if let PhoneticFeatures::Consonant(c) = &config.sounds["b"] {
            assert!(matches!(c.voice, Binary::Plus));
        } else {
            panic!("'b' is not a consonant");
        }
    }

    #[test]
    fn sound_a_is_vowel() {
        let config = load();
        assert!(
            matches!(config.sounds["a"], PhoneticFeatures::Vowel(_)),
            "'a' should be a vowel"
        );
    }

    #[test]
    fn sound_a_high_is_low() {
        let config = load();
        if let PhoneticFeatures::Vowel(v) = &config.sounds["a"] {
            assert!(matches!(v.high, High::Low));
        } else {
            panic!("'a' is not a vowel");
        }
    }

    #[test]
    fn sound_a_back_is_front() {
        let config = load();
        if let PhoneticFeatures::Vowel(v) = &config.sounds["a"] {
            assert!(matches!(v.back, Back::Front));
        } else {
            panic!("'a' is not a vowel");
        }
    }

    #[test]
    fn sound_a_round_is_minus() {
        let config = load();
        if let PhoneticFeatures::Vowel(v) = &config.sounds["a"] {
            assert!(matches!(v.round, Binary::Minus));
        } else {
            panic!("'a' is not a vowel");
        }
    }

    #[test]
    fn sound_a_syllabic_is_plus() {
        let config = load();
        if let PhoneticFeatures::Vowel(v) = &config.sounds["a"] {
            assert!(matches!(v.syllabic, Binary::Plus));
        } else {
            panic!("'a' is not a vowel");
        }
    }

    #[test]
    fn sound_a_long_is_minus() {
        let config = load();
        if let PhoneticFeatures::Vowel(v) = &config.sounds["a"] {
            assert!(matches!(v.long, Binary::Minus));
        } else {
            panic!("'a' is not a vowel");
        }
    }

    #[test]
    fn sound_i_is_vowel() {
        let config = load();
        assert!(
            matches!(config.sounds["i"], PhoneticFeatures::Vowel(_)),
            "'i' should be a vowel"
        );
    }

    #[test]
    fn sound_i_high_is_high() {
        let config = load();
        if let PhoneticFeatures::Vowel(v) = &config.sounds["i"] {
            assert!(matches!(v.high, High::High));
        } else {
            panic!("'i' is not a vowel");
        }
    }

    #[test]
    fn sound_i_back_is_front() {
        let config = load();
        if let PhoneticFeatures::Vowel(v) = &config.sounds["i"] {
            assert!(matches!(v.back, Back::Front));
        } else {
            panic!("'i' is not a vowel");
        }
    }

    #[test]
    fn sound_i_round_is_minus() {
        let config = load();
        if let PhoneticFeatures::Vowel(v) = &config.sounds["i"] {
            assert!(matches!(v.round, Binary::Minus));
        } else {
            panic!("'i' is not a vowel");
        }
    }

    #[test]
    fn rejects_non_toml_extension() {
        let non_toml: Result<RawAlineConfig, String> = parse_toml_file("notatoml.json");
        assert!(non_toml.is_err());
        assert_eq!(non_toml.err().unwrap(), "file must be a .toml");
    }

    #[test]
    fn rejects_missing_file() {
        let nonexistent: Result<RawAlineConfig, String> = parse_toml_file("nonexistent.toml");
        assert!(nonexistent.is_err());
    }
}
