//! aline::config::feature_values
//!
//! This file contains the feature value maps for ALINE.

use enum_map::EnumMap;
use serde::{Deserialize, Serialize};

use super::feature_types::{Back, Binary, High, Manner, Place};

/// Central struct storing all phonetic feature values.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeatureValues {
    pub(crate) place: EnumMap<Place, f32>,
    pub(crate) manner: EnumMap<Manner, f32>,
    pub(crate) high: EnumMap<High, f32>,
    pub(crate) back: EnumMap<Back, f32>,
    pub(crate) binary: EnumMap<Binary, f32>,
}

impl FeatureValues {
    pub fn validate(&self) -> Result<(), String> {
        let check_sum = |sum: f32, name: &str| -> Result<(), String> {
            if (sum - 1.0).abs() < 0.0001 {
                Ok(())
            } else {
                Err(format!("{} values must sum to 1.0, but got {}", name, sum))
            }
        };

        check_sum(self.place.values().copied().sum(), "Place")?;
        check_sum(self.manner.values().copied().sum(), "Manner")?;
        check_sum(self.high.values().copied().sum(), "High")?;
        check_sum(self.back.values().copied().sum(), "Back")?;
        check_sum(self.binary.values().copied().sum(), "Binary")?;

        Ok(())
    }

    pub fn try_new(
        place: EnumMap<Place, f32>,
        manner: EnumMap<Manner, f32>,
        high: EnumMap<High, f32>,
        back: EnumMap<Back, f32>,
        binary: EnumMap<Binary, f32>,
    ) -> Result<Self, String> {
        let values = Self {
            place,
            manner,
            high,
            back,
            binary,
        };
        values.validate()?;
        Ok(values)
    }
}
