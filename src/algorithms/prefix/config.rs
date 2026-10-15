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

    /// Sets whether to perform case-insensitive comparison.
    pub fn set_case_insensitive(&mut self, case_insensitive: bool) {
        self.case_insensitive = case_insensitive;
    }
}
