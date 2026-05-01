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
    pub(crate) costs: Costs,
    /// Whether to perform case-insensitive comparison.
    pub(crate) case_insensitive: bool,
}

impl Levenshtein {
    pub fn new(costs: Costs, case_insensitive: bool) -> Self {
        Self {
            costs,
            case_insensitive,
        }
    }
}
