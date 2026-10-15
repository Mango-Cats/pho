use serde::{Deserialize, Serialize};

use crate::{Error, Result, utils::validate::validated_serde};

/// Configuration for character n-gram TF-IDF cosine similarity.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct CharTfIdf {
    /// N-gram size (e.g. 2 for bigrams, 3 for trigrams).
    pub(crate) n: usize,
    /// Apply sublinear TF scaling: weight = 1 + ln(1 + tf) instead of raw tf.
    /// This dampens the effect of repeated n-grams and distinguishes this
    /// algorithm from the plain Cosine variant of NGram.
    pub(crate) sublinear_tf: bool,
    pub(crate) case_insensitive: bool,
}

validated_serde!(CharTfIdf);

impl CharTfIdf {
    pub fn validate(&self) -> Result<()> {
        if self.n == 0 {
            return Err(Error::InvalidNGramSize(self.n));
        }
        Ok(())
    }

    pub fn try_new(n: usize, sublinear_tf: bool, case_insensitive: bool) -> Result<Self> {
        let cfg = Self {
            n,
            sublinear_tf,
            case_insensitive,
        };
        cfg.validate()?;
        Ok(cfg)
    }

    /// Sets the n-gram size (must be non-zero).
    ///
    /// Returns an error and leaves the config unchanged if the value is invalid.
    pub fn set_n(&mut self, n: usize) -> Result<()> {
        if n == 0 {
            return Err(Error::InvalidNGramSize(n));
        }
        self.n = n;
        Ok(())
    }

    /// Sets whether to apply sublinear TF scaling.
    pub fn set_sublinear_tf(&mut self, sublinear_tf: bool) {
        self.sublinear_tf = sublinear_tf;
    }

    /// Sets whether to perform case-insensitive comparison.
    pub fn set_case_insensitive(&mut self, case_insensitive: bool) {
        self.case_insensitive = case_insensitive;
    }
}
