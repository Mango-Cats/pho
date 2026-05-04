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

/// N-gram similarity with configurable left and right padding.
///
/// The input is padded with `before_padding` spaces in front and
/// `after_padding` spaces at the end before n-grams are extracted.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NGram {
    pub(crate) n: usize,
    pub(crate) before_padding: usize,
    pub(crate) after_padding: usize,
    pub(crate) case_insensitive: bool,
    pub(crate) metric: NGramMetric,
}

impl NGram {
    pub fn validate(&self) -> Result<()> {
        if self.n == 0 {
            return Err(Error::InvalidNGramSize(self.n));
        }

        self.metric.validate()
    }

    pub fn try_new(
        n: usize,
        before_padding: usize,
        after_padding: usize,
        case_insensitive: bool,
        metric: NGramMetric,
    ) -> Result<Self> {
        let config = Self {
            n,
            before_padding,
            after_padding,
            case_insensitive,
            metric,
        };

        config.validate()?;
        Ok(config)
    }
}
