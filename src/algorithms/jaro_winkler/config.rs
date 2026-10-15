use serde::{Deserialize, Serialize};

use crate::{Error, Result, utils::validate::validated_serde};

/// Configuration for the Jaro-Winkler similarity algorithm.
///
/// The Jaro-Winkler metric is a variant of the Jaro distance metric that
/// gives more favorable ratings to strings with common prefixes.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct JaroWinkler {
    /// Scaling factor for the common prefix bonus.
    ///
    /// Standard value is 0.1. Higher values give more weight to matching
    /// prefixes. Must be in range [0.0, 0.25] to ensure the similarity
    /// score remains in [0, 1].
    pub(crate) prefix_scale: f32,

    /// Maximum length of common prefix to consider.
    ///
    /// Standard value is 4. Only the first `max_prefix_length` characters
    /// are considered when computing the prefix bonus.
    pub(crate) max_prefix_length: usize,

    /// Whether to perform case-insensitive comparison.
    pub(crate) case_insensitive: bool,
}

validated_serde!(JaroWinkler);

impl JaroWinkler {
    pub fn validate(&self) -> Result<()> {
        if !(0.0..=0.25).contains(&self.prefix_scale) {
            return Err(Error::InvalidPrefixScale(self.prefix_scale));
        }
        Ok(())
    }

    pub fn try_new(
        prefix_scale: f32,
        max_prefix_length: usize,
        case_insensitive: bool,
    ) -> Result<Self> {
        let config = Self {
            prefix_scale,
            max_prefix_length,
            case_insensitive,
        };

        config.validate()?;
        Ok(config)
    }

    /// Sets the scaling factor for the common prefix bonus (must be in `[0.0, 0.25]`).
    ///
    /// Returns an error and leaves the config unchanged if the value is invalid.
    pub fn set_prefix_scale(&mut self, prefix_scale: f32) -> Result<()> {
        if !(0.0..=0.25).contains(&prefix_scale) {
            return Err(Error::InvalidPrefixScale(prefix_scale));
        }
        self.prefix_scale = prefix_scale;
        Ok(())
    }

    /// Sets the maximum length of common prefix to consider.
    pub fn set_max_prefix_length(&mut self, max_prefix_length: usize) {
        self.max_prefix_length = max_prefix_length;
    }

    /// Sets whether to perform case-insensitive comparison.
    pub fn set_case_insensitive(&mut self, case_insensitive: bool) {
        self.case_insensitive = case_insensitive;
    }
}
