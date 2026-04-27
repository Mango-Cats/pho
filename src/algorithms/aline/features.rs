//! aline::features
//!
//! This file contains the following:
//! 1.  the phonetic features used by the Aline algorithm; examples are
//!     binary (presence or absence), place (of articulation), and
//!     manner (of articulation)
//! 2.  the [`Phoneme`] trait, [`ConsonantFeatures`] and
//!     [`VowelFeatures`] structs, and the [`PhoneticFeatures`] enum.
//!
//! ## How values are stored
//!
//! Each feature has its corresponding `enum` that determines the
//! different possible types of a specific feature. To avoid boilerplate
//! and parallel structs, we use the `enum-map` crate. This allows us
//! to map enum variants directly to `f32` values in a fast, stack-allocated array.
//!
//! ```rust
//! use enum_map::enum_map;
//! use pho::algorithms::aline::features::{Binary, FeatureValues};
//!
//! // Values are initialized using the enum_map! macro.
//! // The compiler enforces that every variant is covered.
//! let binary_map = enum_map! {
//!     Binary::Plus => 0.7,
//!     Binary::Minus => 0.3,
//! };
//!
//! assert_eq!(binary_map[Binary::Plus], 0.7);
//! assert_eq!(binary_map[Binary::Minus], 0.3);
//! ```
//!
//! ## References
//!
//! - https://dl.acm.org/doi/book/10.5555/936774

use crate::algorithms::aline::salience::Salience;
use enum_map::{Enum, EnumMap};
use serde::Deserialize;

/// Has the value of either [`Binary::Plus`] (+) or [`Binary::Minus`]
/// (-). Plus denotes the presence of a feature, while minus denotes
/// the absence.
#[derive(Debug, Clone, Copy, PartialEq, Enum, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Binary {
    Plus,
    Minus,
}

/// Are the possible places of articulation of a specific sound.
#[derive(Debug, Clone, Copy, PartialEq, Enum, Deserialize)]
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

/// Represents the manner of articulation or degree of stricture for a specific sound.
#[derive(Debug, Clone, Copy, PartialEq, Enum, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Enum, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum High {
    High,
    Mid,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Enum, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Back {
    Front,
    Central,
    Back,
}

/// Central struct storing all phonetic feature values.
#[derive(Debug, Clone, Deserialize)]
pub struct FeatureValues {
    pub place: EnumMap<Place, f32>,
    pub manner: EnumMap<Manner, f32>,
    pub high: EnumMap<High, f32>,
    pub back: EnumMap<Back, f32>,
    pub binary: EnumMap<Binary, f32>,
}

impl FeatureValues {
    /// Creates a new FeatureValues struct, panicking immediately
    /// if any of the feature categories do not sum to 1.0 (within a small tolerance).
    pub fn new(
        place: EnumMap<Place, f32>,
        manner: EnumMap<Manner, f32>,
        high: EnumMap<High, f32>,
        back: EnumMap<Back, f32>,
        binary: EnumMap<Binary, f32>,
    ) -> Self {
        let assert_normalized = |sum: f32, name: &str| {
            assert!(
                (sum - 1.0).abs() < 0.0001,
                "FATAL: {} values must sum to 1.0, but got {}",
                name,
                sum
            );
        };

        assert_normalized(place.values().copied().sum(), "Place");
        assert_normalized(manner.values().copied().sum(), "Manner");
        assert_normalized(high.values().copied().sum(), "High");
        assert_normalized(back.values().copied().sum(), "Back");
        assert_normalized(binary.values().copied().sum(), "Binary");

        Self {
            place,
            manner,
            high,
            back,
            binary,
        }
    }
}

/// Shared interface for consonants and vowels. The ALINE sigma function
/// compares any two sounds using whichever features they have in common,
/// so both types need to expose the shared features through a common interface.
pub trait Phoneme {
    fn place(&self) -> &Place;
    fn manner(&self) -> &Manner;
    fn syllabic(&self) -> &Binary;
    fn voice(&self) -> &Binary;
    fn nasal(&self) -> &Binary;
    fn retroflex(&self) -> &Binary;
    fn lateral(&self) -> &Binary;
    fn is_vowel(&self) -> bool;
}

/// Stores the phonetic features of a consonant sound.
#[derive(Debug, Clone)]
pub struct ConsonantFeatures {
    pub aspirated: Binary,
    pub lateral: Binary,
    pub manner: Manner,
    pub nasal: Binary,
    pub place: Place,
    pub retroflex: Binary,
    pub syllabic: Binary,
    pub voice: Binary,
}

impl Phoneme for ConsonantFeatures {
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
    fn is_vowel(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct VowelFeatures {
    pub back: Back,
    pub high: High,
    pub lateral: Binary,
    pub long: Binary,
    pub nasal: Binary,
    pub retroflex: Binary,
    pub round: Binary,
    pub syllabic: Binary,
    pub voice: Binary,
    pub place: Place,
    pub manner: Manner,
}

impl Phoneme for VowelFeatures {
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
    fn is_vowel(&self) -> bool {
        true
    }
}

/// A product type over consonant and vowel features. Pattern match on
/// this when you need vowel-specific features; use `.as_phoneme()` when
/// you only need the shared interface.
#[derive(Debug, Clone)]
pub enum PhoneticFeatures {
    Consonant(ConsonantFeatures),
    Vowel(VowelFeatures),
}

impl PhoneticFeatures {
    /// Returns a reference to the shared `Phoneme` trait object so callers
    /// don't need to match on the variant just to read common features.
    pub fn as_phoneme(&self) -> &dyn Phoneme {
        match self {
            PhoneticFeatures::Consonant(c) => c,
            PhoneticFeatures::Vowel(v) => v,
        }
    }

    /// Computes the phonetic similarity between two sounds (the ALINE `sigma`
    /// function). Returns a score in the range [0, C_sub] where a higher score
    /// means the two sounds are more similar.
    ///
    /// The score is computed as:
    /// `sigma(p, q) = C_sub - diff(p, q)`
    ///
    /// where `diff` sums the salience-weighted absolute difference between
    /// each shared feature value.
    pub fn sigma(
        &self,
        other: &PhoneticFeatures,
        values: &FeatureValues,
        salience: &Salience,
        c_sub: i32,
    ) -> f32 {
        let p = self.as_phoneme();
        let q = other.as_phoneme();

        // Values are retrieved safely in O(1) time by indexing the EnumMap
        let diff = feature_difference(
            values.place[*p.place()],
            values.place[*q.place()],
            salience.place,
        ) + feature_difference(
            values.manner[*p.manner()],
            values.manner[*q.manner()],
            salience.manner,
        ) + feature_difference(
            values.binary[*p.syllabic()],
            values.binary[*q.syllabic()],
            salience.syllabic,
        ) + feature_difference(
            values.binary[*p.voice()],
            values.binary[*q.voice()],
            salience.voice,
        ) + feature_difference(
            values.binary[*p.nasal()],
            values.binary[*q.nasal()],
            salience.nasal,
        ) + feature_difference(
            values.binary[*p.retroflex()],
            values.binary[*q.retroflex()],
            salience.retroflex,
        ) + feature_difference(
            values.binary[*p.lateral()],
            values.binary[*q.lateral()],
            salience.lateral,
        );

        // vowel-only features
        let vowel_diff = match (self, other) {
            (PhoneticFeatures::Vowel(a), PhoneticFeatures::Vowel(b)) => {
                feature_difference(values.high[a.high], values.high[b.high], salience.high)
                    + feature_difference(values.back[a.back], values.back[b.back], salience.back)
                    + feature_difference(
                        values.binary[a.round],
                        values.binary[b.round],
                        salience.round,
                    )
                    + feature_difference(
                        values.binary[a.long],
                        values.binary[b.long],
                        salience.long,
                    )
            }
            _ => 0.0,
        };

        (c_sub as f32) - (diff + vowel_diff)
    }
}

/// Computes the salience-weighted absolute difference between two feature values.
/// This mirrors: `salience[f] * |similarity_matrix[p[f]] - similarity_matrix[q[f]]|`
#[inline]
fn feature_difference(a: f32, b: f32, salience: u32) -> f32 {
    salience as f32 * (a - b).abs()
}
