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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Aline {
    pub costs: Costs,
    pub salience: Salience,
    pub values: FeatureValues,
    pub sounds: HashMap<String, PhoneticFeatures>,
    pub epsilon: f32,
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
        };
        config.validate()?;
        Ok(config)
    }
}
