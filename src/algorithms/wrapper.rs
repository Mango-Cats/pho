use crate::algorithms::{
    aline::{self, config::AlineConfig},
    editex::{self, config::EditexConfig},
    jaro_winkler::{self, config::JaroWinklerConfig},
    levenshtein::{self, config::LevenshteinConfig},
};

use super::AlgorithmTrait;

/// Generates a thin wrapper type around an algorithm module.
///
/// Each wrapper stores the algorithm's config and exposes a unified
/// `AlgorithmTrait` implementation.
macro_rules! define_algorithm {
    ($name:ident, $config:ty, $module:ident) => {
        /// Algorithm wrapper that carries its configuration.
        pub struct $name {
            pub config: $config,
        }

        impl $name {
            /// Build a new algorithm wrapper from a config value.
            pub fn new(config: &$config) -> Self {
                Self {
                    config: config.clone(),
                }
            }
        }

        impl AlgorithmTrait for $name {
            fn similarity(&self, x: &str, y: &str) -> Result<f32, String> {
                $module::similarity(x, y, &self.config).map_err(|e| e.to_string())
            }
        }
    };
}

define_algorithm!(AlineAlgorithm, AlineConfig, aline);
define_algorithm!(EditexAlgorithm, EditexConfig, editex);
define_algorithm!(JaroWinklerAlgorithm, JaroWinklerConfig, jaro_winkler);
define_algorithm!(LevenshteinAlgorithm, LevenshteinConfig, levenshtein);
