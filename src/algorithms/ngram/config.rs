use serde::{Deserialize, Serialize};

use crate::{
    Error, Result, algorithms::ngram::metric::NGramMetric, utils::validate::validated_serde,
};

/// N-gram similarity with configurable left and right padding.
///
/// The input is padded with `before_padding` spaces in front and
/// `after_padding` spaces at the end before n-grams are extracted.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct NGram {
    pub(crate) n: usize,
    pub(crate) before_padding: usize,
    pub(crate) after_padding: usize,
    pub(crate) case_insensitive: bool,
    pub(crate) metric: NGramMetric,
}

validated_serde!(NGram);

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

    /// Sets the number of padding spaces added before the input.
    pub fn set_before_padding(&mut self, before_padding: usize) {
        self.before_padding = before_padding;
    }

    /// Sets the number of padding spaces added after the input.
    pub fn set_after_padding(&mut self, after_padding: usize) {
        self.after_padding = after_padding;
    }

    /// Sets whether to perform case-insensitive comparison.
    pub fn set_case_insensitive(&mut self, case_insensitive: bool) {
        self.case_insensitive = case_insensitive;
    }

    /// Sets the n-gram similarity metric.
    ///
    /// Returns an error and leaves the config unchanged if the value is invalid.
    pub fn set_metric(&mut self, metric: NGramMetric) -> Result<()> {
        metric.validate()?;
        self.metric = metric;
        Ok(())
    }
}
