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
    Dental,
    Alveolar,
    Retroflex,
    PalatoAlveolar,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
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
    Trill,
    Tap,
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
    Mid,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Enum, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Back {
    Front,
    Central,
    Back,
}
