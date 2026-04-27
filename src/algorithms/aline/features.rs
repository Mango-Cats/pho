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

/// The possible places of articulation of a sound.
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

/// The manner of articulation or degree of stricture for a sound.
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
    /// Creates a new `FeatureValues`, panicking if any category does not
    /// sum to 1.0 (within a small tolerance).
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

/// Features shared by every sound. [`Phoneme`] is implemented here once
/// to avoid duplicating getters across [`ConsonantFeatures`] and [`VowelFeatures`].
#[derive(Debug, Clone, Deserialize)]
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
#[derive(Debug, Clone, Deserialize)]
pub struct ConsonantFeatures {
    #[serde(flatten)]
    pub common: CommonFeatures,
    pub aspirated: Binary,
}

/// Stores the phonetic features of a vowel sound.
#[derive(Debug, Clone, Deserialize)]
pub struct VowelFeatures {
    #[serde(flatten)]
    pub common: CommonFeatures,
    pub back: Back,
    pub high: High,
    pub long: Binary,
    pub round: Binary,
}

/// Shared interface for the sigma function. Implemented once on
/// [`CommonFeatures`] and accessed via [`PhoneticFeatures::common()`].
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
#[derive(Debug, Clone)]
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

    /// Computes the phonetic similarity between two sounds (the ALINE `sigma`
    /// function). Returns a score in the range [0, C_sub] where a higher score
    /// means the two sounds are more similar.
    ///
    /// The score is computed as:
    /// `sigma(p, q) = C_sub - diff(p, q)`
    ///
    /// where `diff` sums the salience-weighted absolute difference between
    /// each common feature value.
    pub fn sigma(
        &self,
        other: &PhoneticFeatures,
        values: &FeatureValues,
        salience: &Salience,
        c_sub: i32,
    ) -> f32 {
        let p = self.common();
        let q = other.common();

        let diff = fd(
            values.place[*p.place()],
            values.place[*q.place()],
            salience.place,
        ) + fd(
            values.manner[*p.manner()],
            values.manner[*q.manner()],
            salience.manner,
        ) + fd(
            values.binary[*p.syllabic()],
            values.binary[*q.syllabic()],
            salience.syllabic,
        ) + fd(
            values.binary[*p.voice()],
            values.binary[*q.voice()],
            salience.voice,
        ) + fd(
            values.binary[*p.nasal()],
            values.binary[*q.nasal()],
            salience.nasal,
        ) + fd(
            values.binary[*p.retroflex()],
            values.binary[*q.retroflex()],
            salience.retroflex,
        ) + fd(
            values.binary[*p.lateral()],
            values.binary[*q.lateral()],
            salience.lateral,
        );

        let vowel_diff = match (self, other) {
            (PhoneticFeatures::Vowel(a), PhoneticFeatures::Vowel(b)) => {
                fd(values.high[a.high], values.high[b.high], salience.high)
                    + fd(values.back[a.back], values.back[b.back], salience.back)
                    + fd(
                        values.binary[a.round],
                        values.binary[b.round],
                        salience.round,
                    )
                    + fd(values.binary[a.long], values.binary[b.long], salience.long)
            }
            _ => 0.0,
        };

        (c_sub as f32) - (diff + vowel_diff)
    }
}

/// Salience-weighted absolute difference between two feature values.
/// Mirrors: `salience[f] * |matrix[p[f]] - matrix[q[f]]|`
#[inline]
fn fd(a: f32, b: f32, salience: u32) -> f32 {
    salience as f32 * (a - b).abs()
}
