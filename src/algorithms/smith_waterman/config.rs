use serde::{Deserialize, Serialize};

use crate::{
    Error, Result,
    utils::validate::{non_negative, validated_serde},
};

/// Configuration for Smith-Waterman local sequence alignment.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct SmithWaterman {
    /// Score awarded for a matching character pair. Must be positive.
    pub(crate) match_score: f32,
    /// Penalty applied for a mismatching character pair (stored as positive).
    pub(crate) mismatch_penalty: f32,
    /// Linear gap penalty per gap character (stored as positive).
    pub(crate) gap_penalty: f32,
    pub(crate) case_insensitive: bool,
}

validated_serde!(SmithWaterman);

fn check_match_score(match_score: f32) -> Result<()> {
    if match_score.is_finite() && match_score > 0.0 {
        Ok(())
    } else {
        Err(Error::InvalidMatchScore(match_score))
    }
}

impl SmithWaterman {
    pub fn validate(&self) -> Result<()> {
        check_match_score(self.match_score)?;
        non_negative("mismatch_penalty", self.mismatch_penalty)?;
        non_negative("gap_penalty", self.gap_penalty)
    }

    pub fn try_new(
        match_score: f32,
        mismatch_penalty: f32,
        gap_penalty: f32,
        case_insensitive: bool,
    ) -> Result<Self> {
        let config = Self {
            match_score,
            mismatch_penalty,
            gap_penalty,
            case_insensitive,
        };
        config.validate()?;
        Ok(config)
    }

    /// Sets the score awarded for a matching character pair (must be positive).
    ///
    /// Returns an error and leaves the config unchanged if the value is invalid.
    pub fn set_match_score(&mut self, match_score: f32) -> Result<()> {
        check_match_score(match_score)?;
        self.match_score = match_score;
        Ok(())
    }

    /// Sets the penalty for a mismatching character pair (must be non-negative).
    ///
    /// Returns an error and leaves the config unchanged if the value is invalid.
    pub fn set_mismatch_penalty(&mut self, mismatch_penalty: f32) -> Result<()> {
        non_negative("mismatch_penalty", mismatch_penalty)?;
        self.mismatch_penalty = mismatch_penalty;
        Ok(())
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
