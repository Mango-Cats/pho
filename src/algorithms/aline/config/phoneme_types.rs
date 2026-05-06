use serde::{Deserialize, Serialize};

use crate::algorithms::aline::config::{
    Back, Binary, High, Manner, Phoneme, Place,
    feature_types::{Airstream, Phonation, SecondaryArticulation},
};

/// Features shared by every sound.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommonFeatures {
    pub(crate) place: Place,
    pub(crate) manner: Manner,
    pub(crate) syllabic: Binary,
    pub(crate) voice: Binary,
    pub(crate) nasal: Binary,
    pub(crate) retroflex: Binary,
    pub(crate) lateral: Binary,
    pub(crate) phonation: Phonation,
    pub(crate) airstream: Airstream,
    pub(crate) secondary: SecondaryArticulation,
}

impl CommonFeatures {
    pub fn new(
        place: Place,
        manner: Manner,
        syllabic: Binary,
        voice: Binary,
        nasal: Binary,
        retroflex: Binary,
        lateral: Binary,
        phonation: Phonation,
        airstream: Airstream,
        secondary: SecondaryArticulation,
    ) -> Self {
        Self {
            place,
            manner,
            syllabic,
            voice,
            nasal,
            retroflex,
            lateral,
            phonation,
            airstream,
            secondary,
        }
    }
}

/// Stores the phonetic features of a consonant sound.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConsonantFeatures {
    #[serde(flatten)]
    pub(crate) common: CommonFeatures,
    pub(crate) aspirated: Binary,
}

impl ConsonantFeatures {
    /// Infallible constructor for ConsonantFeatures
    pub fn new(common: CommonFeatures, aspirated: Binary) -> Self {
        Self { common, aspirated }
    }
}

/// Stores the phonetic features of a vowel sound.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VowelFeatures {
    #[serde(flatten)]
    pub(crate) common: CommonFeatures,
    pub(crate) back: Back,
    pub(crate) high: High,
    pub(crate) long: Binary,
    pub(crate) round: Binary,
}

impl VowelFeatures {
    /// Infallible constructor for VowelFeatures
    pub fn new(
        common: CommonFeatures,
        back: Back,
        high: High,
        long: Binary,
        round: Binary,
    ) -> Self {
        Self {
            common,
            back,
            high,
            long,
            round,
        }
    }
}

/// An enum consonant and vowel features.
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
