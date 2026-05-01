use crate::algorithms::AlgorithmTrait;

/// Bundle an algorithm with its config and ensemble weight.
pub struct WeightedAlgorithm {
    pub algorithm: Box<dyn AlgorithmTrait>,
    pub weight: f32,
}

impl WeightedAlgorithm {
    pub fn new<A>(algorithm: A, weight: f32) -> Self
    where
        A: AlgorithmTrait + 'static,
    {
        Self {
            algorithm: Box::new(algorithm),
            weight,
        }
    }
}

/// An ensemble algorithm is a vector of [`WeightedAlgorithm`]
/// instances.
pub struct EnsembleAlgorithm {
    pub algorithms: Vec<WeightedAlgorithm>,
}

impl EnsembleAlgorithm {
    /// Validate non-empty, finite, and normalized weights.
    pub fn validate(&self) -> Result<(), String> {
        if self.algorithms.is_empty() {
            return Err("ensemble algorithms must be non-empty".to_string());
        }

        let mut total = 0.0f32;
        for weighted in &self.algorithms {
            if !weighted.weight.is_finite() {
                return Err("ensemble weight must be finite".to_string());
            }
            if weighted.weight < 0.0 {
                return Err("ensemble weight must be non-negative".to_string());
            }
            total += weighted.weight;
        }

        if (total - 1.0).abs() >= 0.0001 {
            return Err("ensemble weights must sum to 1.0".to_string());
        }

        Ok(())
    }
}
