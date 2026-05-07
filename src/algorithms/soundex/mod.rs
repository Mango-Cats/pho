//! soundex
//!
//! American Soundex phonetic similarity.
//!
//! Both strings are encoded into a four-character Soundex code and then
//! compared. Two strings that sound alike under English pronunciation rules
//! receive the same code.
//!
//! ## Similarity modes
//!
//! - **Binary** (default): 1.0 if both codes are identical, 0.0 otherwise.
//! - **Soft**: position-wise agreement across the four code characters,
//!   giving partial credit when prefix letter or digit groups partly match.

pub mod config;
mod encode;

use crate::{algorithms::Algorithm, error::Result};
use config::{Soundex, SoundexMode};
use encode::{code_similarity, soundex};

impl Algorithm for Soundex {
    fn similarity(&self, x: &str, y: &str) -> Result<f32> {
        let (xs, ys) = if self.case_insensitive {
            (x.to_lowercase(), y.to_lowercase())
        } else {
            (x.to_string(), y.to_string())
        };

        let code_x = soundex(&xs);
        let code_y = soundex(&ys);

        if code_x.is_empty() || code_y.is_empty() {
            return Ok(0.0);
        }

        let score = match self.mode {
            SoundexMode::Binary => {
                if code_x == code_y { 1.0 } else { 0.0 }
            }
            SoundexMode::Soft => code_similarity(&code_x, &code_y),
        };

        Ok(score)
    }
}
