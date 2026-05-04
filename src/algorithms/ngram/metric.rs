use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// Similarity metric used by the n-gram algorithm.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NGramMetric {
    Dice,
    Jaccard,
    Overlap,
    Tversky { alpha: f32, beta: f32 },
    Cosine,
}

impl NGramMetric {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Tversky { alpha, beta } => {
                if !alpha.is_finite() || *alpha < 0.0 {
                    return Err(Error::InvalidTverskyParameter {
                        name: "alpha",
                        value: *alpha,
                    });
                }

                if !beta.is_finite() || *beta < 0.0 {
                    return Err(Error::InvalidTverskyParameter {
                        name: "beta",
                        value: *beta,
                    });
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }
}
