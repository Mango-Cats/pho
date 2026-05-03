use serde::{Deserialize, Serialize};

/// Cost configuration for Levenshtein distance operations.
///
/// These costs determine the penalty for each edit operation when
/// computing the distance between two strings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Costs {
    /// Cost for inserting a character.
    pub(crate) insert: f32,
    /// Cost for deleting a character.
    pub(crate) delete: f32,
    /// Cost for substituting one character with another.
    pub(crate) substitute: f32,
}

impl Costs {
    pub fn new(insert: f32, delete: f32, substitute: f32) -> Self {
        Self {
            insert,
            delete,
            substitute,
        }
    }
}

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
