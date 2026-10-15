use serde::{Deserialize, Serialize};

use crate::Result;

/// Configuration for Kondrak's BI-SIM bigram similarity.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BiSim {
    /// Whether to perform case-insensitive comparison.
    pub(crate) case_insensitive: bool,
}

impl BiSim {
    pub fn try_new(case_insensitive: bool) -> Result<Self> {
        let config = Self { case_insensitive };
        Ok(config)
    }

    /// Sets whether to perform case-insensitive comparison.
    pub fn set_case_insensitive(&mut self, case_insensitive: bool) {
        self.case_insensitive = case_insensitive;
    }
}
