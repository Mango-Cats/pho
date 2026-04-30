use serde::{Deserialize, Serialize};

use super::feature_types::{Back, Binary, High, Manner, Place};
use super::phoneme_trait::Phoneme;

/// Features shared by every sound. [`Phoneme`] is implemented here once
/// to avoid duplicating getters across [`ConsonantFeatures`] and [`VowelFeatures`].
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommonFeatures {
    pub place: Place,
    pub manner: Manner,
    pub syllabic: Binary,
    pub voice: Binary,
    pub nasal: Binary,
    pub retroflex: Binary,
    pub lateral: Binary,
}

/// Stores the phonetic features of a consonant sound.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConsonantFeatures {
    #[serde(flatten)]
    pub common: CommonFeatures,
    pub aspirated: Binary,
}

/// Stores the phonetic features of a vowel sound.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VowelFeatures {
    #[serde(flatten)]
    pub common: CommonFeatures,
    pub back: Back,
    pub high: High,
    pub long: Binary,
    pub round: Binary,
}

/// An enum consonant and vowel features. Pattern match on
/// this when you need sound-specific features; use `.common()` when you
/// only need the common interface.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PhoneticFeatures {
    Consonant(ConsonantFeatures),
    Vowel(VowelFeatures),
}

impl PhoneticFeatures {
    /// Returns the common features as a [`Phoneme`] trait object.
    pub fn common(&self) -> &dyn Phoneme {
        match self {
            PhoneticFeatures::Consonant(c) => &c.common,
            PhoneticFeatures::Vowel(v) => &v.common,
        }
    }

    pub fn is_vowel(&self) -> bool {
        matches!(self, PhoneticFeatures::Vowel(_))
    }

    pub fn is_consonant(&self) -> bool {
        !self.is_vowel()
    }
}
