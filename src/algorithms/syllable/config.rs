use serde::{Deserialize, Serialize};

/// Set-theoretic metric for syllable bigram overlap.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SyllableMetric {
    /// Dice coefficient: 2|A∩B| / (|A| + |B|).
    #[default]
    Dice,
    /// Jaccard index: |A∩B| / |A∪B|.
    Jaccard,
    /// Overlap coefficient: |A∩B| / min(|A|, |B|).
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
        Self { case_insensitive, metric }
    }
}
