use super::cost::Cost;
use serde::{Deserialize, Serialize};

/// Configuration for the Levenshtein distance algorithm.
///
/// This structure holds the cost parameters that control how
/// different edit operations are weighted when computing string distance.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LevenshteinConfig {
    /// Edit operation costs.
    pub costs: Cost,
    /// Whether to perform case-insensitive comparison.
    pub case_insensitive: bool,
}
