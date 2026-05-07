use serde::{Deserialize, Serialize};

/// Configuration for Double Metaphone phonetic similarity.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DoubleMetaphone {
    pub(crate) case_insensitive: bool,
    /// Maximum code length considered per string. 0 means unlimited.
    pub(crate) max_code_length: usize,
}

impl DoubleMetaphone {
    pub fn new(case_insensitive: bool, max_code_length: usize) -> Self {
        Self { case_insensitive, max_code_length }
    }
}
