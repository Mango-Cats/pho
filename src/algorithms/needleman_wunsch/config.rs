use serde::{Deserialize, Serialize};

use crate::{
    Result,
    utils::validate::{non_negative, validated_serde},
};

/// Configuration for Needleman-Wunsch global alignment with a drug-name
/// substitution matrix.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct NeedlemanWunsch {
    /// Linear gap penalty per gap character (stored as positive).
    pub(crate) gap_penalty: f32,
    pub(crate) case_insensitive: bool,
}

validated_serde!(NeedlemanWunsch);

impl NeedlemanWunsch {
    pub fn validate(&self) -> Result<()> {
        non_negative("gap_penalty", self.gap_penalty)
    }

    pub fn try_new(gap_penalty: f32, case_insensitive: bool) -> Result<Self> {
        let config = Self {
            gap_penalty,
            case_insensitive,
        };
        config.validate()?;
        Ok(config)
    }

    /// Sets the linear gap penalty per gap character (must be non-negative).
    ///
    /// Returns an error and leaves the config unchanged if the value is invalid.
    pub fn set_gap_penalty(&mut self, gap_penalty: f32) -> Result<()> {
        non_negative("gap_penalty", gap_penalty)?;
        self.gap_penalty = gap_penalty;
        Ok(())
    }

    /// Sets whether to perform case-insensitive comparison.
    pub fn set_case_insensitive(&mut self, case_insensitive: bool) {
        self.case_insensitive = case_insensitive;
    }
}
