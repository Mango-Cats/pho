use crate::algorithms::{Algorithm, AlgorithmConfig};

/// Bundle an algorithm with its config and ensemble weight.
#[derive(Debug)]
pub struct WeightedAlgorithm {
    pub algorithm: Algorithm,
    pub config: AlgorithmConfig,
    pub weight: f32,
}

impl WeightedAlgorithm {
    pub fn new(algorithm: Algorithm, config: AlgorithmConfig, weight: f32) -> Self {
        Self {
            algorithm,
            config,
            weight,
        }
    }
}

/// An ensemble algorithm is a vector of [`WeightedAlgorithm`]
/// instances.
pub struct EnsembleAlgorithm {
    pub algorithms: Vec<WeightedAlgorithm>,
}
