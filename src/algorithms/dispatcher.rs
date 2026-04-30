use crate::algorithms::{aline, editex, jaro_winkler, levenshtein};

use super::{AlgorithmConfig, Algorithm};

impl Algorithm {
    pub fn similarity(
        self,
        x: &str,
        y: &str,
        similarity_config: Option<&AlgorithmConfig>,
    ) -> Result<f32, String> {
        match self {
            Algorithm::Aline => {
                let Some(AlgorithmConfig::AlineConfig(config)) = similarity_config else {
                    return Err("ALINE requires an AlineConfig".to_string());
                };
                aline::similarity(x, y, config).map_err(|e| e.to_string())
            }
            Algorithm::Editex => {
                let Some(AlgorithmConfig::EditexConfig(config)) = similarity_config else {
                    return Err("Editex requires an EditexConfig".to_string());
                };
                editex::similarity(x, y, config).map_err(|e| e.to_string())
            }
            Algorithm::JaroWinkler => {
                let Some(AlgorithmConfig::JaroWinklerConfig(config)) = similarity_config else {
                    return Err("JaroWinkler requires a JaroWinklerConfig".to_string());
                };
                jaro_winkler::similarity(x, y, config).map_err(|e| e.to_string())
            }
            Algorithm::Levenshtein => {
                let Some(AlgorithmConfig::LevenshteinConfig(config)) = similarity_config else {
                    return Err("Levenshtein requires a LevenshteinConfig".to_string());
                };
                levenshtein::similarity(x, y, config).map_err(|e| e.to_string())
            }
        }
    }
}
