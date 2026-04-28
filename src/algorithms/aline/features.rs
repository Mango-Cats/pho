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
