//! aline::config::feature_values
//!
//! This file contains the feature value maps for ALINE.

use crate::{
    algorithms::aline::config::feature_types::{Airstream, Phonation, SecondaryArticulation},
    error::{Error::InvalidFeatureValue, Result},
};
use enum_map::{EnumArray, EnumMap};
use serde::{Deserialize, Serialize};

use super::feature_types::{Back, Binary, High, Manner, Place};

/// Central struct storing all phonetic feature values.
///
/// Each value is a position on a `[0.0, 1.0]` scale (for example, `stop` is
/// 1.0 and `low_vowel` is 0.0); ALINE scores the difference between two
/// positions weighted by the feature's salience. The one exception is
/// `place.vowel`, which follows NLTK in using `-1.0` and only has to be finite.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeatureValues {
    pub(crate) place: EnumMap<Place, f32>,
    pub(crate) manner: EnumMap<Manner, f32>,
    pub(crate) high: EnumMap<High, f32>,
    pub(crate) back: EnumMap<Back, f32>,
    pub(crate) binary: EnumMap<Binary, f32>,
    pub(crate) phonation: EnumMap<Phonation, f32>,
    pub(crate) airstream: EnumMap<Airstream, f32>,
    pub(crate) secondary: EnumMap<SecondaryArticulation, f32>,
}

impl FeatureValues {
    pub fn validate(&self) -> Result<()> {
        let vowel_place = self.place[Place::Vowel];
        if !vowel_place.is_finite() {
            return Err(InvalidFeatureValue {
                feature: "Place",
                name: format!("{:?}", Place::Vowel),
                value: vowel_place,
            });
        }
        let mut place = self.place;
        place[Place::Vowel] = 0.0;
        check_range(&place, "Place")?;
        check_range(&self.manner, "Manner")?;
        check_range(&self.high, "High")?;
        check_range(&self.back, "Back")?;
        check_range(&self.binary, "Binary")?;
        check_range(&self.phonation, "Phonation")?;
        check_range(&self.airstream, "Airstream")?;
        check_range(&self.secondary, "Secondary")?;

        Ok(())
    }

    pub fn try_new(
        place: EnumMap<Place, f32>,
        manner: EnumMap<Manner, f32>,
        high: EnumMap<High, f32>,
        back: EnumMap<Back, f32>,
        binary: EnumMap<Binary, f32>,
        phonation: EnumMap<Phonation, f32>,
        airstream: EnumMap<Airstream, f32>,
        secondary: EnumMap<SecondaryArticulation, f32>,
    ) -> Result<Self> {
        let values = Self {
            place,
            manner,
            high,
            back,
            binary,
            phonation,
            airstream,
            secondary,
        };
        values.validate()?;
        Ok(values)
    }
}

/// Checks that every value of a feature map is within `[0.0, 1.0]`.
fn check_range<K>(values: &EnumMap<K, f32>, feature: &'static str) -> Result<()>
where
    K: EnumArray<f32> + std::fmt::Debug,
{
    for (key, &value) in values {
        if !(0.0..=1.0).contains(&value) {
            return Err(InvalidFeatureValue {
                feature,
                name: format!("{key:?}"),
                value,
            });
        }
    }
    Ok(())
}
