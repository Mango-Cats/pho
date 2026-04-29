use super::features::{Back, Binary, High, Manner, Place};
use serde::{Deserialize, Serialize};

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

/// Shared interface for accessing common features.
///
/// Implemented once on [`CommonFeatures`] and accessed via
/// [`PhoneticFeatures::common()`].
pub trait Phoneme {
    fn place(&self) -> &Place;
    fn manner(&self) -> &Manner;
    fn syllabic(&self) -> &Binary;
    fn voice(&self) -> &Binary;
    fn nasal(&self) -> &Binary;
    fn retroflex(&self) -> &Binary;
    fn lateral(&self) -> &Binary;
}

impl Phoneme for CommonFeatures {
    fn place(&self) -> &Place {
        &self.place
    }
    fn manner(&self) -> &Manner {
        &self.manner
    }
    fn syllabic(&self) -> &Binary {
        &self.syllabic
    }
    fn voice(&self) -> &Binary {
        &self.voice
    }
    fn nasal(&self) -> &Binary {
        &self.nasal
    }
    fn retroflex(&self) -> &Binary {
        &self.retroflex
    }
    fn lateral(&self) -> &Binary {
        &self.lateral
    }
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
        return !self.is_vowel();
    }
}
