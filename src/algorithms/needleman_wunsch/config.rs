use serde::{Deserialize, Serialize};

/// Configuration for Needleman-Wunsch global alignment with a drug-name
/// substitution matrix.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NeedlemanWunsch {
    /// Linear gap penalty per gap character (stored as positive).
    pub(crate) gap_penalty: f32,
    pub(crate) case_insensitive: bool,
}

impl NeedlemanWunsch {
    pub fn new(gap_penalty: f32, case_insensitive: bool) -> Self {
        Self { gap_penalty, case_insensitive }
    }
}
