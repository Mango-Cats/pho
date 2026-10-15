use serde::{Deserialize, Serialize};

/// Set-theoretic metric for syllable bigram overlap.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SyllableMetric {
    /// Dice coefficient: 2 * |A and B| / (|A| + |B|).
    #[default]
    Dice,
    /// Jaccard index: |A and B| / |A or B|.
    Jaccard,
    /// Overlap coefficient: |A and B| / min(|A|, |B|).
    Overlap,
}

/// Configuration for syllable-bigram overlap similarity.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Syllable {
    pub(crate) case_insensitive: bool,
    pub(crate) metric: SyllableMetric,
}

impl Syllable {
    pub fn new(case_insensitive: bool, metric: SyllableMetric) -> Self {
        Self {
            case_insensitive,
            metric,
        }
    }

    /// Sets whether to perform case-insensitive comparison.
    pub fn set_case_insensitive(&mut self, case_insensitive: bool) {
        self.case_insensitive = case_insensitive;
    }

    /// Sets the syllable bigram overlap metric.
    pub fn set_metric(&mut self, metric: SyllableMetric) {
        self.metric = metric;
    }
}
