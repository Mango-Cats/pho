//! editex
//!
//! A Rust implementation of the Editex phonetic similarity algorithm.
//!
//! ## What Editex computes
//!
//! - `edit_distance(a, b)` computes the Editex distance between two strings.
//! - `similarity(a, b)` computes a normalized score in $[0, 1]$:
//!   $$\text{similarity}(a,b) = 1 - \frac{\text{edit\_distance}(a,b)}{\text{max\_distance}(a,b)}$$
//!
//! The edit costs are driven by phonetic groups in the config. Characters in the
//! same group are cheaper to substitute, insert, or delete than characters in
//! different groups.
//!
//! ## Example
//!
//! ```rust
//! use pho::{algorithms::{Editex, AlgorithmTrait}, config_io::import};
//!
//! let algo: Editex = import("tests/config_sample_editex.toml").unwrap();
//! let similarity = algo.similarity("Smith", "Smyth").unwrap();
//! assert!((0.0..=1.0).contains(&similarity));
//! ```

pub mod config;
pub mod edit;
pub mod group;

mod distance;
mod tokenize;

use crate::algorithms::{
    AlgorithmTrait,
    editex::{
        distance::{edit_distance, total_delete_cost},
        tokenize::tokenize_and_validate,
    },
    errors::AlgorithmError,
};

use config::Editex;

impl AlgorithmTrait for Editex {
    fn similarity(&self, x: &str, y: &str) -> Result<f32, AlgorithmError> {
        let x_chars = tokenize_and_validate(x, self, "x")?;
        let y_chars = tokenize_and_validate(y, self, "y")?;

        let distance = edit_distance(&x_chars, &y_chars, self);
        let max_distance = total_delete_cost(&x_chars, self) + total_delete_cost(&y_chars, self);

        if max_distance == 0.0 {
            return Ok(1.0);
        }

        let similarity = 1.0 - (distance / max_distance);
        Ok(similarity.clamp(0.0, 1.0))
    }
}
