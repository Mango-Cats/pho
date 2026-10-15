use serde::{Deserialize, Serialize};

/// How Soundex codes are compared to produce a similarity score.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SoundexMode {
    /// 1.0 if codes are identical, 0.0 otherwise.
    #[default]
    Binary,
    /// Position-wise agreement on the four-character code, giving partial credit.
    Soft,
}

/// Configuration for Soundex phonetic similarity.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Soundex {
    pub(crate) case_insensitive: bool,
    pub(crate) mode: SoundexMode,
}

impl Soundex {
    pub fn new(case_insensitive: bool, mode: SoundexMode) -> Self {
        Self {
            case_insensitive,
            mode,
        }
    }

    /// Sets whether to perform case-insensitive comparison.
    pub fn set_case_insensitive(&mut self, case_insensitive: bool) {
        self.case_insensitive = case_insensitive;
    }

    /// Sets how Soundex codes are compared.
    pub fn set_mode(&mut self, mode: SoundexMode) {
        self.mode = mode;
    }
}
