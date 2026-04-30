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

/// Common interface for similarity algorithms.
///
/// Implementations should return a normalized similarity score and convert
/// algorithm-specific errors into a string message.
///
/// # Examples
///
/// ```no_run
/// use pho::algorithms::{
///     config_io::parse_toml_file,
///     AlineAlgorithm,
///     SimilarityAlgorithmTrait,
/// };
///
/// fn main() -> Result<(), String> {
///     let config = parse_toml_file("tests/config_sample_aline.toml")?;
///     let algo = AlineAlgorithm::new(config);
///     let _score = algo.similarity("s", "s")?;
///     Ok(())
/// }
/// ```
pub trait SimilarityAlgorithmTrait {
    fn similarity(&self, x: &str, y: &str) -> Result<f32, String>;
}

/// Generates a thin wrapper type around an algorithm module.
///
/// Each wrapper stores the algorithm's config and exposes a unified
/// `SimilarityAlgorithmTrait` implementation.
macro_rules! define_algorithm {
    ($name:ident, $config:ty, $module:ident) => {
        /// Algorithm wrapper that carries its configuration.
        pub struct $name {
            pub config: $config,
        }

        impl $name {
            /// Build a new algorithm wrapper from a config value.
            pub fn new(config: $config) -> Self {
                Self { config }
            }
        }

        impl SimilarityAlgorithmTrait for $name {
            fn similarity(&self, x: &str, y: &str) -> Result<f32, String> {
                $module::similarity(x, y, &self.config).map_err(|e| e.to_string())
            }
        }
    };
}

define_algorithm!(AlineAlgorithm, aline::config::AlineConfig, aline);
define_algorithm!(EditexAlgorithm, editex::config::EditexConfig, editex);

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
