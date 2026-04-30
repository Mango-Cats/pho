pub mod aline;
pub mod config_io;
pub mod editex;
pub mod validation;

pub enum SimilarityAlgorithm {
    Aline,
    Editex,
}

pub enum AlgorithmConfig {
    AlineConfig(aline::config::AlineConfig),
    EditexConfig(editex::config::EditexConfig),
}

impl SimilarityAlgorithm {
    pub fn similarity(
        self,
        x: &str,
        y: &str,
        similarity_config: Option<&AlgorithmConfig>,
    ) -> Result<f32, String> {
        match self {
            SimilarityAlgorithm::Aline => {
                let Some(AlgorithmConfig::AlineConfig(config)) = similarity_config else {
                    return Err("ALINE requires an AlineConfig".to_string());
                };
                aline::similarity(x, y, config).map_err(|e| e.to_string())
            }
            SimilarityAlgorithm::Editex => {
                let Some(AlgorithmConfig::EditexConfig(config)) = similarity_config else {
                    return Err("Editex requires an EditexConfig".to_string());
                };
                editex::similarity(x, y, config).map_err(|e| e.to_string())
            }
        }
    }
}
