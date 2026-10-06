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

use crate::{Error, Result};
pub use cost::Costs;
pub use feature_types::{Back, Binary, High, Manner, Place};
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

impl Aline {
    pub fn validate(&self) -> Result<()> {
        self.values.validate()?;
        if self.epsilon < 0.0 {
            return Err(Error::NegativeEpsilon(self.epsilon));
        }
        Ok(())
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
}
