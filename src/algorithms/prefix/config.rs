use serde::{Deserialize, Serialize};

/// Configuration for the prefix similarity algorithm.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Prefix {
    /// Whether to perform case-insensitive comparison.
    pub case_insensitive: bool,
}

impl Prefix {
    pub fn new(case_insensitive: bool) -> Self {
        Self { case_insensitive }
    }
}
