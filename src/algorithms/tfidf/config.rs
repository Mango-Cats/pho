use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// Configuration for character n-gram TF-IDF cosine similarity.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CharTfIdf {
    /// N-gram size (e.g. 2 for bigrams, 3 for trigrams).
    pub(crate) n: usize,
    /// Apply sublinear TF scaling: weight = 1 + ln(1 + tf) instead of raw tf.
    /// This dampens the effect of repeated n-grams and distinguishes this
    /// algorithm from the plain Cosine variant of NGram.
    pub(crate) sublinear_tf: bool,
    pub(crate) case_insensitive: bool,
}

impl CharTfIdf {
    pub fn validate(&self) -> Result<()> {
        if self.n == 0 {
            return Err(Error::InvalidNGramSize(self.n));
        }
        Ok(())
    }

    pub fn try_new(n: usize, sublinear_tf: bool, case_insensitive: bool) -> Result<Self> {
        let cfg = Self { n, sublinear_tf, case_insensitive };
        cfg.validate()?;
        Ok(cfg)
    }
}
