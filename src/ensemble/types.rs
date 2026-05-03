// src/ensemble/config.rs

use crate::algorithms::Algorithm;

pub struct WeightedAlgorithm {
    pub algorithm: Box<dyn Algorithm>,
    pub weight: f32,
}

impl WeightedAlgorithm {
    pub fn new<A>(algorithm: A, weight: f32) -> Self
    where
        A: Algorithm + 'static,
    {
        Self {
            algorithm: Box::new(algorithm),
            weight,
        }
    }
}

pub struct EnsembleAlgorithm {
    pub algorithms: Vec<WeightedAlgorithm>,
    pub is_probability_distribution: bool,
    pub allow_negative_weights: bool,
}

impl EnsembleAlgorithm {
    pub fn new_uniform_probability(algorithms: Vec<Box<dyn Algorithm>>) -> crate::Result<Self> {
        if algorithms.is_empty() {
            return Err(crate::Error::EmptyEnsemble);
        }

        let n = algorithms.len() as f32;
        let weight = 1.0 / n;

        let algorithms = algorithms
            .into_iter()
            .map(|a| WeightedAlgorithm {
                algorithm: a,
                weight,
            })
            .collect();

        Ok(Self {
            algorithms,
            is_probability_distribution: true,
            allow_negative_weights: false,
        })
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.algorithms.is_empty() {
            return Err(crate::Error::EmptyEnsemble);
        }

        let mut total = 0.0f32;

        for weighted in &self.algorithms {
            if !weighted.weight.is_finite() {
                return Err(crate::Error::NonFiniteWeight(weighted.weight));
            }

            if self.is_probability_distribution && weighted.weight < 0.0 {
                return Err(crate::Error::NegativeWeight(weighted.weight));
            }

            if !self.is_probability_distribution
                && !self.allow_negative_weights
                && weighted.weight < 0.0
            {
                return Err(crate::Error::NegativeWeight(weighted.weight));
            }

            total += weighted.weight;
        }

        if self.is_probability_distribution {
            if (total - 1.0).abs() >= 0.0001 {
                return Err(crate::Error::WeightsDoNotSumToOne(total));
            }
        } else if total == 0.0 {
            return Err(crate::Error::InvalidWeight(0.0));
        }

        Ok(())
    }

    pub fn try_new(
        algorithms: Vec<WeightedAlgorithm>,
        is_probability_distribution: bool,
        allow_negative_weights: bool,
    ) -> crate::Result<Self> {
        let ensemble = Self {
            algorithms,
            is_probability_distribution,
            allow_negative_weights,
        };
        ensemble.validate()?;
        Ok(ensemble)
    }
}
