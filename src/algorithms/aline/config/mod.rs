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
///   (`ˈ`) assigns weight 1.0 and secondary stress (`ˌ`) assigns weight 0.5
///   to the segments that follow them; the difference in stress between aligned
///   segments is penalised by `salience.stress`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AlineVariant {
    #[default]
    Kondrak,
    MangoCats,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Aline {
    pub costs: Costs,
    pub salience: Salience,
    pub values: FeatureValues,
    pub sounds: HashMap<String, PhoneticFeatures>,
    pub epsilon: f32,
    pub variant: AlineVariant,
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
        };
        config.validate()?;
        Ok(config)
    }
}
