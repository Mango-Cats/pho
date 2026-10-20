//! aline::config
//!
//! This module holds configuration values and phonetic feature models for ALINE.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod cost;
mod feature_types;
mod feature_values;
mod phoneme_trait;
mod phoneme_types;
pub mod salience;

use crate::{Error, Result, utils::validate::validated_serde};
pub use cost::Costs;
pub use enum_map;
pub use feature_types::{
    Airstream, Back, Binary, High, Manner, Phonation, Place, SecondaryArticulation,
};
pub use feature_values::FeatureValues;
pub use phoneme_trait::Phoneme;
pub use phoneme_types::{CommonFeatures, ConsonantFeatures, PhoneticFeatures, VowelFeatures};
pub use salience::Salience;

/// Selects which variant of the ALINE algorithm to use.
///
/// - `Kondrak`: the original Kondrak (2002) algorithm. Stress markers in the
///   IPA input are ignored entirely.
/// - `MangoCats`: extends Kondrak with a stress-salience term. Primary stress
///   assigns weight 1.0 and secondary stress assigns weight 0.5
///   to the segments of the syllable it marks (up to the next period, the next
///   stress mark, or the end of the word). The difference in stress between
///   aligned segments is penalized by `salience.stress`. See
///   [`StressScopeFallback`] and [`StressOn`].
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AlineVariant {
    #[default]
    Kondrak,
    #[serde(alias = "mangocats")]
    MangoCats,
}

/// What to do when a word carries a stress mark but no `.` syllable
/// boundaries, so the extent of the stressed syllable is unknown.
#[derive(Debug, Clone, Copy, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StressScopeFallback {
    /// Return [`Error::AmbiguousStressScope`].
    #[default]
    Error,
    /// Legacy behaviour: the stress level applies to every following segment
    /// until the next stress mark (or the end of the word).
    UntilNextMark,
    /// The stress level applies only to the first vowel segment after the
    /// mark. Every other segment of the word is unstressed.
    NextVowel,
}

/// Which aligned pairs the stress term is applied to.
#[derive(Debug, Clone, Copy, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StressOn {
    /// Every aligned pair, consonants included.
    All,
    /// Only pairs where both sides are vowels.
    #[default]
    Vowels,
}

/// Dynamic-programming alignment mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AlignmentMode {
    /// Smith-Waterman style local alignment (Kondrak and NLTK behavior).
    #[default]
    Local,
    /// Needleman-Wunsch style global alignment: leading and trailing indels are
    /// charged, and the score is read from the bottom-right cell.
    Global,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct Aline {
    pub costs: Costs,
    pub salience: Salience,
    pub values: FeatureValues,
    pub sounds: HashMap<String, PhoneticFeatures>,
    pub epsilon: f32,
    pub variant: AlineVariant,
    /// Allow the tokenizer to produce multi-grapheme vowel entries
    /// (such as diphthongs). When `false`, only single-grapheme vowels
    /// and consonants are matched.
    #[serde(default)]
    pub merge_diphthongs: bool,
    /// See [`StressScopeFallback`]. Only relevant in the MangoCats variant.
    #[serde(default)]
    pub stress_scope_fallback: StressScopeFallback,
    /// See [`StressOn`]. Only relevant in the MangoCats variant.
    #[serde(default)]
    pub stress_on: StressOn,
    /// See [`AlignmentMode`].
    #[serde(default)]
    pub alignment_mode: AlignmentMode,
}

validated_serde!(Aline);

fn check_epsilon(epsilon: f32) -> Result<()> {
    if epsilon.is_finite() && epsilon >= 0.0 {
        Ok(())
    } else {
        Err(Error::NegativeEpsilon(epsilon))
    }
}

impl Aline {
    pub fn validate(&self) -> Result<()> {
        self.costs.validate()?;
        self.salience.validate()?;
        self.values.validate()?;
        check_epsilon(self.epsilon)
    }

    pub fn try_new(
        costs: Costs,
        salience: Salience,
        values: FeatureValues,
        sounds: HashMap<String, PhoneticFeatures>,
        epsilon: f32,
    ) -> Result<Self> {
        let config = Self {
            costs,
            salience,
            values,
            sounds,
            epsilon,
            variant: AlineVariant::Kondrak,
            merge_diphthongs: false,
            stress_scope_fallback: StressScopeFallback::default(),
            stress_on: StressOn::default(),
            alignment_mode: AlignmentMode::default(),
        };
        config.validate()?;
        Ok(config)
    }

    /// Sets the ALINE cost constants.
    pub fn set_costs(&mut self, costs: Costs) {
        self.costs = costs;
    }

    /// Sets the feature salience weights.
    pub fn set_salience(&mut self, salience: Salience) {
        self.salience = salience;
    }

    /// Sets the phonetic feature values.
    ///
    /// Returns an error and leaves the config unchanged if the value is invalid.
    pub fn set_values(&mut self, values: FeatureValues) -> Result<()> {
        values.validate()?;
        self.values = values;
        Ok(())
    }

    /// Sets the phoneme inventory.
    pub fn set_sounds(&mut self, sounds: HashMap<String, PhoneticFeatures>) {
        self.sounds = sounds;
    }

    /// Sets the epsilon used for near-optimal alignments (must be finite and non-negative).
    ///
    /// Returns an error and leaves the config unchanged if the value is invalid.
    pub fn set_epsilon(&mut self, epsilon: f32) -> Result<()> {
        check_epsilon(epsilon)?;
        self.epsilon = epsilon;
        Ok(())
    }

    /// Sets the ALINE variant.
    pub fn set_variant(&mut self, variant: AlineVariant) {
        self.variant = variant;
    }

    /// Sets whether multi-grapheme vowel entries may be matched.
    pub fn set_merge_diphthongs(&mut self, merge_diphthongs: bool) {
        self.merge_diphthongs = merge_diphthongs;
    }

    /// Sets the stress scope fallback.
    pub fn set_stress_scope_fallback(&mut self, stress_scope_fallback: StressScopeFallback) {
        self.stress_scope_fallback = stress_scope_fallback;
    }

    /// Sets which aligned pairs the stress term applies to.
    pub fn set_stress_on(&mut self, stress_on: StressOn) {
        self.stress_on = stress_on;
    }

    /// Sets the alignment mode.
    pub fn set_alignment_mode(&mut self, alignment_mode: AlignmentMode) {
        self.alignment_mode = alignment_mode;
    }
}
