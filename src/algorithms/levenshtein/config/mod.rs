use serde::{Deserialize, Serialize};

pub mod costs;

pub use costs::Costs;

/// Configuration for the Levenshtein distance algorithm.
///
/// This structure holds the cost parameters that control how
/// different edit operations are weighted when computing string distance.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Levenshtein {
    /// Edit operation costs.
    pub costs: Costs,
    /// Whether to perform case-insensitive comparison.
    pub case_insensitive: bool,
}
