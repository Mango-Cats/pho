use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownTokenError {
    pub token: String,
    /// 0-based position in the tokenized sequence.
    pub position: usize,
    /// Which input this occurred in (e.g. "x" or "y").
    pub input_name: &'static str,
    /// Human-readable context for where the token was expected.
    pub context: &'static str,
}

impl fmt::Display for UnknownTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Unknown token '{}' at position {} in {} (not found in {})",
            self.token, self.position, self.input_name, self.context
        )
    }
}

impl Error for UnknownTokenError {}
