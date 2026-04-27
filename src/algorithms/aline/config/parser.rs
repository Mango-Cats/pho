//! aline::config::parser
//!
//! This module maps a TOML configuration document into ALINE runtime types.
//! Generic file parsing lives in `crate::config` so this module can stay
//! focused on ALINE-specific schema and validation.

use serde::Deserialize;
use std::collections::HashMap;

use super::types::AlineConfig;
use crate::algorithms::aline::{
    cost::Costs,
    features::{
        Back, BackValues, Binary, BinaryValues, ConsonantFeatures, FeatureValues, High, HighValues,
        Manner, MannerValues, PhoneticFeatures, Place, PlaceValues, VowelFeatures,
    },
    salience::Salience,
};
use crate::config::parse_toml_file;

#[derive(Debug, Deserialize)]
struct RawAlineConfig {
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
    fn into_config(self) -> AlineConfig {
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

/// Create an AlineConfig from a TOML file.
///
/// This preserves the existing API while delegating generic TOML parsing to
/// the shared parser and converting the document into ALINE types.
pub fn config_from_toml(file_name: &str) -> Result<AlineConfig, String> {
    let raw: RawAlineConfig = parse_toml_file(file_name)?;
    Ok(raw.into_config())
}
