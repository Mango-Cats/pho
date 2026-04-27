//! aline::config::types
//!
//! This file contains the `AlineConfig` struct which stores similarity
//! values and phonetic features that is used by the Aline algorithm.
use crate::algorithms::aline::{
    cost::Costs,
    features::{FeatureValues, PhoneticFeatures},
    salience::Salience,
};

use std::collections::HashMap;

/// This struct stores the similarity values and phonetic features
/// that is used by the Aline algorithm.
pub struct AlineConfig {
    pub costs: Costs,
    pub salience: Salience,
    pub values: FeatureValues,
    pub sounds: HashMap<String, PhoneticFeatures>,
}

// aline::config::parser
//
// This module maps a TOML configuration document into ALINE runtime types.
// Generic file parsing lives in `crate::config` so this module can stay
// focused on ALINE-specific schema and validation.
//
// FIXME: The stuff below this used to belong in config::parser.rs
// but i put it here to remove redundancy. But this needs a lot of fixes.
// Especially on duplicates and kaing things more succinct.

use serde::Deserialize;

use crate::algorithms::aline::features::{
    Back, BackValues, Binary, BinaryValues, ConsonantFeatures, High, HighValues, Manner,
    MannerValues, Place, PlaceValues, VowelFeatures,
};

#[derive(Debug, Deserialize)]
pub struct RawAlineConfig {
    costs: Costs,
    salience: Salience,
    place_values: PlaceValues,
    manner_values: MannerValues,
    height_values: HighValues,
    backness_values: BackValues,
    binary_values: BinaryValues,
    sounds: HashMap<String, RawSound>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawSound {
    Consonant {
        aspirated: Binary,
        lateral: Binary,
        manner: Manner,
        nasal: Binary,
        place: Place,
        retroflex: Binary,
        syllabic: Binary,
        voice: Binary,
    },
    Vowel {
        back: Back,
        high: High,
        round: Binary,
        long: Binary,
        place: Place,
        manner: Manner,
        lateral: Binary,
        nasal: Binary,
        retroflex: Binary,
        syllabic: Binary,
        voice: Binary,
    },
}

impl RawAlineConfig {
    pub fn into_config(self) -> AlineConfig {
        let sounds = self
            .sounds
            .into_iter()
            .map(|(symbol, raw_sound)| (symbol, raw_sound.into_features()))
            .collect();

        AlineConfig {
            costs: self.costs,
            salience: self.salience,
            values: FeatureValues {
                place: self.place_values,
                manner: self.manner_values,
                high: self.height_values,
                back: self.backness_values,
                binary: self.binary_values,
            },
            sounds,
        }
    }
}

impl RawSound {
    fn into_features(self) -> PhoneticFeatures {
        match self {
            RawSound::Consonant {
                aspirated,
                lateral,
                manner,
                nasal,
                place,
                retroflex,
                syllabic,
                voice,
            } => PhoneticFeatures::Consonant(ConsonantFeatures {
                aspirated,
                lateral,
                manner,
                nasal,
                place,
                retroflex,
                syllabic,
                voice,
            }),
            RawSound::Vowel {
                back,
                high,
                round,
                long,
                place,
                manner,
                lateral,
                nasal,
                retroflex,
                syllabic,
                voice,
            } => PhoneticFeatures::Vowel(VowelFeatures {
                back,
                high,
                round,
                long,
                place,
                manner,
                lateral,
                nasal,
                retroflex,
                syllabic,
                voice,
            }),
        }
    }
}
