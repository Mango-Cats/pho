//! Builds an [`Aline`] entirely through the public API (no TOML) and checks
//! that it scores the same as the equivalent TOML config.

use std::collections::HashMap;

use pho::{
    algorithms::{
        Algorithm, Aline,
        aline::config::{
            Airstream, AlineVariant, Back, Binary, CommonFeatures, ConsonantFeatures, Costs,
            FeatureValues, High, Manner, Phonation, PhoneticFeatures, Place, Salience,
            SecondaryArticulation, VowelFeatures, enum_map::enum_map,
        },
    },
    utils::io::import,
};

const TOML_PATH: &str = "algorithm_configs/fil/filipino_aline.toml";

const PAIRS: &[(&str, &str)] = &[
    ("pa", "ba"),
    ("ka", "pa"),
    ("pi", "pa"),
    ("bi", "ki"),
    ("pak", "bak"),
    ("kapi", "kabi"),
    ("ab", "ap"),
];

fn common(place: Place, manner: Manner, syllabic: Binary, voice: Binary) -> CommonFeatures {
    CommonFeatures::new(
        place,
        manner,
        syllabic,
        voice,
        Binary::Minus,
        Binary::Minus,
        Binary::Minus,
        Phonation::Modal,
        Airstream::Pulmonic,
        SecondaryArticulation::None,
    )
}

fn vowel(manner: Manner, back: Back, high: High) -> PhoneticFeatures {
    PhoneticFeatures::Vowel(VowelFeatures::new(
        common(Place::Vowel, manner, Binary::Plus, Binary::Plus),
        back,
        high,
        Binary::Minus,
        Binary::Minus,
    ))
}

fn consonant(place: Place, voice: Binary) -> PhoneticFeatures {
    PhoneticFeatures::Consonant(ConsonantFeatures::new(
        common(place, Manner::Stop, Binary::Minus, voice),
        Binary::Minus,
    ))
}

/// Mirrors `filipino_aline.toml`, restricted to `a`, `i`, `p`, `b`, `k`.
fn build() -> Aline {
    let costs = Costs::try_new(-10, 35, 45, 5).unwrap();
    let salience = Salience::new(5, 40, 50, 5, 20, 10, 10, 5, 10, 3, 2, 2, 10, 10, 10, 0);
    let values = FeatureValues::try_new(
        enum_map! {
            Place::Bilabial => 1.0,
            Place::Labiodental => 0.95,
            Place::Linguolabial => 0.92,
            Place::Dental => 0.9,
            Place::Alveolar => 0.85,
            Place::Retroflex => 0.8,
            Place::PalatoAlveolar => 0.75,
            Place::AlveoloPalatal => 0.72,
            Place::Palatal => 0.7,
            Place::Velar => 0.6,
            Place::Uvular => 0.5,
            Place::Pharyngeal => 0.3,
            Place::Epiglottal => 0.2,
            Place::Glottal => 0.1,
            Place::Labiovelar => 1.0,
            Place::Vowel => -1.0,
        },
        enum_map! {
            Manner::Stop => 1.0,
            Manner::Affricate => 0.9,
            Manner::Fricative => 0.85,
            Manner::LateralFricative => 0.8,
            Manner::Trill => 0.7,
            Manner::Tap => 0.65,
            Manner::LateralFlap => 0.6,
            Manner::Approximant => 0.6,
            Manner::HighVowel => 0.4,
            Manner::MidVowel => 0.2,
            Manner::LowVowel => 0.0,
            Manner::Vowel => 0.5,
        },
        enum_map! {
            High::High => 1.0,
            High::NearHigh => 0.85,
            High::HighMid => 0.65,
            High::Mid => 0.5,
            High::LowMid => 0.35,
            High::NearLow => 0.15,
            High::Low => 0.0,
        },
        enum_map! {
            Back::Front => 1.0,
            Back::NearFront => 0.75,
            Back::Central => 0.5,
            Back::NearBack => 0.25,
            Back::Back => 0.0,
        },
        enum_map! {
            Binary::Plus => 1.0,
            Binary::Minus => 0.0,
        },
        enum_map! {
            Phonation::Modal => 1.0,
            Phonation::Breathy => 0.75,
            Phonation::Creaky => 0.5,
            Phonation::Stiff => 0.25,
            Phonation::Slack => 0.0,
        },
        enum_map! {
            Airstream::Pulmonic => 1.0,
            Airstream::Ejective => 0.66,
            Airstream::Implosive => 0.33,
            Airstream::Click => 0.0,
        },
        enum_map! {
            SecondaryArticulation::None => 1.0,
            SecondaryArticulation::Labialized => 0.75,
            SecondaryArticulation::Palatalized => 0.5,
            SecondaryArticulation::Velarized => 0.25,
            SecondaryArticulation::Pharyngealized => 0.0,
        },
    )
    .unwrap();

    let sounds = HashMap::from([
        (
            "a".to_string(),
            vowel(Manner::LowVowel, Back::Central, High::Low),
        ),
        (
            "i".to_string(),
            vowel(Manner::HighVowel, Back::Front, High::High),
        ),
        ("p".to_string(), consonant(Place::Bilabial, Binary::Minus)),
        ("b".to_string(), consonant(Place::Bilabial, Binary::Plus)),
        ("k".to_string(), consonant(Place::Velar, Binary::Minus)),
    ]);

    let mut aline = Aline::try_new(costs, salience, values, sounds, 0.001).unwrap();
    aline.set_variant(AlineVariant::MangoCats);
    aline
}

#[test]
fn rust_built_config_matches_toml() {
    let built = build();
    let loaded: Aline = match import(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    };

    for &(x, y) in PAIRS {
        let b = built.similarity(x, y).unwrap();
        let l = loaded.similarity(x, y).unwrap();
        assert_eq!(b, l, "similarity({x}, {y})");

        let b = built.similarity_unclamped(x, y).unwrap();
        let l = loaded.similarity_unclamped(x, y).unwrap();
        assert_eq!(b, l, "similarity_unclamped({x}, {y})");
    }
}
