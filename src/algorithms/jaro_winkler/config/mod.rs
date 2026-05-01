use serde::{Deserialize, Serialize};

/// Configuration for the Jaro-Winkler similarity algorithm.
///
/// The Jaro-Winkler metric is a variant of the Jaro distance metric that
/// gives more favorable ratings to strings with common prefixes.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JaroWinkler {
    /// Scaling factor for the common prefix bonus.
    ///
    /// Standard value is 0.1. Higher values give more weight to matching
    /// prefixes. Must be in range [0.0, 0.25] to ensure the similarity
    /// score remains in [0, 1].
    pub prefix_scale: f32,

    /// Maximum length of common prefix to consider.
    ///
    /// Standard value is 4. Only the first `max_prefix_length` characters
    /// are considered when computing the prefix bonus.
    pub max_prefix_length: usize,

    /// Whether to perform case-insensitive comparison.
    pub case_insensitive: bool,
}

impl JaroWinkler {
    /// Validate documented invariants for this config.
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=0.25).contains(&self.prefix_scale) {
            return Err("prefix_scale must be in [0.0, 0.25]".to_string());
        }

        Ok(())
    }
}
