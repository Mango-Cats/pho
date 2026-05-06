use crate::ensemble::{config::EnsembleConfig, weighted_function::WeightedFunction};

pub struct EnsembleAlgorithm {
    pub algorithms: Vec<WeightedFunction>,
    pub mode: EnsembleConfig,
}

impl EnsembleAlgorithm {
    pub fn try_new(algorithms: Vec<WeightedFunction>, mode: EnsembleConfig) -> crate::Result<Self> {
        let ensemble = Self { algorithms, mode };
        ensemble.validate()?;
        Ok(ensemble)
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.algorithms.is_empty() {
            return Err(crate::Error::EmptyEnsemble);
        }

        let mut total = 0.0f32;
        let mut has_negative = false;

        for weighted in &self.algorithms {
            if !weighted.weight.is_finite() {
                return Err(crate::Error::NonFiniteWeight(weighted.weight));
            }
            if weighted.weight < 0.0 {
                has_negative = true;
            }
            total += weighted.weight;
        }

        match self.mode {
            EnsembleConfig::Linear => {
                if total == 0.0 {
                    return Err(crate::Error::InvalidWeight(0.0));
                }
            }
            EnsembleConfig::Conical => {
                if has_negative {
                    return Err(crate::Error::NegativeWeight);
                }
                if total == 0.0 {
                    return Err(crate::Error::InvalidWeight(0.0));
                }
            }
            EnsembleConfig::Affine => {
                if (total - 1.0).abs() >= 0.0001 {
                    return Err(crate::Error::WeightsDoNotSumToOne(total));
                }
            }
            EnsembleConfig::Convex => {
                if has_negative {
                    return Err(crate::Error::NegativeWeight);
                }
                if (total - 1.0).abs() >= 0.0001 {
                    return Err(crate::Error::WeightsDoNotSumToOne(total));
                }
            }
        }

        Ok(())
    }
}
