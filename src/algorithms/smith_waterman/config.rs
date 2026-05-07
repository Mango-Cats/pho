use serde::{Deserialize, Serialize};

/// Configuration for Smith-Waterman local sequence alignment.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SmithWaterman {
    /// Score awarded for a matching character pair.
    pub(crate) match_score: f32,
    /// Penalty applied for a mismatching character pair (stored as positive).
    pub(crate) mismatch_penalty: f32,
    /// Linear gap penalty per gap character (stored as positive).
    pub(crate) gap_penalty: f32,
    pub(crate) case_insensitive: bool,
}

impl SmithWaterman {
    pub fn new(
        match_score: f32,
        mismatch_penalty: f32,
        gap_penalty: f32,
        case_insensitive: bool,
    ) -> Self {
        Self { match_score, mismatch_penalty, gap_penalty, case_insensitive }
    }
}
