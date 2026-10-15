use serde::{Deserialize, Serialize};

/// Configuration for Metaphone phonetic similarity.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metaphone {
    pub(crate) case_insensitive: bool,
    /// Maximum length of the phonetic code to compare. 0 means unlimited.
    pub(crate) max_code_length: usize,
}

impl Metaphone {
    pub fn new(case_insensitive: bool, max_code_length: usize) -> Self {
        Self {
            case_insensitive,
            max_code_length,
        }
    }

    /// Sets whether to perform case-insensitive comparison.
    pub fn set_case_insensitive(&mut self, case_insensitive: bool) {
        self.case_insensitive = case_insensitive;
    }

    /// Sets the maximum length of the phonetic code to compare (0 means unlimited).
    pub fn set_max_code_length(&mut self, max_code_length: usize) {
        self.max_code_length = max_code_length;
    }
}
