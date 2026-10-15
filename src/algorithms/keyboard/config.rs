use serde::{Deserialize, Serialize};

use crate::{
    Result,
    utils::validate::{non_negative, validated_serde},
};

/// Configuration for keyboard-proximity edit-distance similarity.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct Keyboard {
    /// Cost per insertion (gap in one of the strings). Must be non-negative.
    pub(crate) insert_cost: f32,
    /// Cost per deletion. Must be non-negative.
    pub(crate) delete_cost: f32,
    /// Scale factor for substitution cost derived from keyboard distance.
    /// substitution_cost = (distance / MAX_KEY_DISTANCE) * scale.
    /// A scale of 1.0 keeps substitution cost in [0, 1]. Must be non-negative.
    pub(crate) substitution_scale: f32,
    pub(crate) case_insensitive: bool,
}

validated_serde!(Keyboard);

impl Keyboard {
    pub fn validate(&self) -> Result<()> {
        non_negative("insert_cost", self.insert_cost)?;
        non_negative("delete_cost", self.delete_cost)?;
        non_negative("substitution_scale", self.substitution_scale)
    }

    pub fn try_new(
        insert_cost: f32,
        delete_cost: f32,
        substitution_scale: f32,
        case_insensitive: bool,
    ) -> Result<Self> {
        let config = Self {
            insert_cost,
            delete_cost,
            substitution_scale,
            case_insensitive,
        };
        config.validate()?;
        Ok(config)
    }

    /// Sets the cost per insertion (must be non-negative).
    ///
    /// Returns an error and leaves the config unchanged if the value is invalid.
    pub fn set_insert_cost(&mut self, insert_cost: f32) -> Result<()> {
        non_negative("insert_cost", insert_cost)?;
        self.insert_cost = insert_cost;
        Ok(())
    }

    /// Sets the cost per deletion (must be non-negative).
    ///
    /// Returns an error and leaves the config unchanged if the value is invalid.
    pub fn set_delete_cost(&mut self, delete_cost: f32) -> Result<()> {
        non_negative("delete_cost", delete_cost)?;
        self.delete_cost = delete_cost;
        Ok(())
    }

    /// Sets the scale factor for keyboard-distance substitution cost (must be non-negative).
    ///
    /// Returns an error and leaves the config unchanged if the value is invalid.
    pub fn set_substitution_scale(&mut self, substitution_scale: f32) -> Result<()> {
        non_negative("substitution_scale", substitution_scale)?;
        self.substitution_scale = substitution_scale;
        Ok(())
    }

    /// Sets whether to perform case-insensitive comparison.
    pub fn set_case_insensitive(&mut self, case_insensitive: bool) {
        self.case_insensitive = case_insensitive;
    }
}
