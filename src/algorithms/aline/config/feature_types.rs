//! aline::config::feature_types
//!
//! This file contains the phonetic feature enums used by ALINE.

use enum_map::Enum;
use serde::{Deserialize, Serialize};

/// Has the value of either [`Binary::Plus`] (+) or [`Binary::Minus`]
/// (-). Plus denotes the presence of a feature, while minus denotes
/// the absence.
#[derive(Debug, Clone, Copy, PartialEq, Enum, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Binary {
    Plus,
    Minus,
}

/// The possible places of articulation of a sound.
#[derive(Debug, Clone, Copy, PartialEq, Enum, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Place {
    Bilabial,
    Labiodental,
    Linguolabial,
    Dental,
    Alveolar,
    PalatoAlveolar,
    Retroflex,
    AlveoloPalatal,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Epiglottal,
    Glottal,
    Labiovelar,
    Vowel,
}

/// The manner of articulation or degree of stricture for a sound.
#[derive(Debug, Clone, Copy, PartialEq, Enum, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Manner {
    Stop,
    Affricate,
    Fricative,
    LateralFricative,
    Trill,
    Tap,
    LateralFlap,
    Approximant,
    HighVowel,
    MidVowel,
    LowVowel,
    Vowel,
}

#[derive(Debug, Clone, Copy, PartialEq, Enum, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum High {
    High,
    NearHigh,
    HighMid,
    Mid,
    LowMid,
    NearLow,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Enum, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Back {
    Front,
    NearFront,
    Central,
    NearBack,
    Back,
}

/// Phonation (Voice quality markers like breathy, creaky)
#[derive(Debug, Clone, Copy, PartialEq, Enum, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Phonation {
    Modal,
    Breathy,
    Creaky,
    Stiff,
    Slack,
}

/// Airstream Mechanisms (Implosives, Ejectives, Clicks)
#[derive(Debug, Clone, Copy, PartialEq, Enum, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Airstream {
    Pulmonic,
    Ejective,
    Implosive,
    Click,
}

/// Secondary Articulations (Labialized, Palatalized, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Enum, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SecondaryArticulation {
    None,
    Labialized,
    Palatalized,
    Velarized,
    Pharyngealized,
}
