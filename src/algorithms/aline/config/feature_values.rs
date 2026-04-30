//! aline::config::feature_values
//!
//! This file contains the feature value maps for ALINE.

use enum_map::EnumMap;
use serde::{Deserialize, Serialize};

use super::feature_types::{Back, Binary, High, Manner, Place};

/// Central struct storing all phonetic feature values.
#[derive(Debug, Clone, Deserialize, Serialize)]
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

    /// Validate that each category is normalized to 1.0 (within tolerance).
    pub fn validate(&self) -> Result<(), String> {
        let check_normalized = |sum: f32, name: &str| {
            if (sum - 1.0).abs() < 0.0001 {
                Ok(())
            } else {
                Err(format!("{} values must sum to 1.0, but got {}", name, sum))
            }
        };

        check_normalized(self.place.values().copied().sum(), "Place")?;
        check_normalized(self.manner.values().copied().sum(), "Manner")?;
        check_normalized(self.high.values().copied().sum(), "High")?;
        check_normalized(self.back.values().copied().sum(), "Back")?;
        check_normalized(self.binary.values().copied().sum(), "Binary")?;

        Ok(())
    }
}
