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
//! use pho::{algorithms::{Editex, Algorithm}, utils::io::import};
//!
//! let algo: Editex = import("tests/config_sample_editex.toml").unwrap();
//! let similarity = algo.similarity("Smith", "Smyth").unwrap();
//! assert!((0.0..=1.0).contains(&similarity));
//! ```

pub mod config;
pub mod edit;
pub mod group;

mod distance;

use crate::{algorithms::Algorithm, error::Result, utils::validate::validate_tokens};

use config::Editex;
use distance::{distance, total_delete_cost};

impl Algorithm for Editex {
    fn distance(&self, x: &str, y: &str) -> Result<f32> {
        let x_chars = validate_tokens(
            x.chars()
                .map(|c| c.to_ascii_lowercase())
                .filter(|c| c.is_ascii_alphabetic()),
            "x",
            "Editex config groups",
            |symbol| self.group.contains_key(symbol),
        )?;

        let y_chars = validate_tokens(
            y.chars()
                .map(|c| c.to_ascii_lowercase())
                .filter(|c| c.is_ascii_alphabetic()),
            "y",
            "Editex config groups",
            |symbol| self.group.contains_key(symbol),
        )?;

        Ok(distance(&x_chars, &y_chars, self))
    }

    fn normalized_distance(&self, x: &str, y: &str) -> Result<f32> {
        let x_chars = validate_tokens(
            x.chars()
                .map(|c| c.to_ascii_lowercase())
                .filter(|c| c.is_ascii_alphabetic()),
            "x",
            "Editex config groups",
            |symbol| self.group.contains_key(symbol),
        )?;

        let y_chars = validate_tokens(
            y.chars()
                .map(|c| c.to_ascii_lowercase())
                .filter(|c| c.is_ascii_alphabetic()),
            "y",
            "Editex config groups",
            |symbol| self.group.contains_key(symbol),
        )?;

        let distance = distance(&x_chars, &y_chars, self);
        let max_distance = total_delete_cost(&x_chars, self) + total_delete_cost(&y_chars, self);

        if max_distance == 0.0 {
            return Ok(0.0);
        }

        Ok((distance / max_distance).clamp(0.0, 1.0))
    }

    fn similarity(&self, x: &str, y: &str) -> Result<f32> {
        let normalized_distance = self.normalized_distance(x, y)?;
        Ok((1.0 - normalized_distance).clamp(0.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::{algorithms::Editex, error::Result, utils::io::import};
    use core::panic;

    const TOML_PATH: &str = "tests/config_sample_editex.toml";

    fn load() -> Editex {
        match import(TOML_PATH) {
            Ok(config) => config,
            Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
        }
    }

    #[test]
    fn cost_same() {
        assert_eq!(load().costs.same, 1.0);
    }

    #[test]
    fn cost_diff() {
        assert_eq!(load().costs.diff, 2.0);
    }

    #[test]
    fn group_has_expected_size() {
        assert_eq!(load().group.len(), 26);
    }

    #[test]
    fn group_a() {
        assert_eq!(load().group[&'a'], vec![0]);
    }

    #[test]
    fn group_c() {
        assert_eq!(load().group[&'c'], vec![2, 9]);
    }

    #[test]
    fn group_h() {
        assert_eq!(load().group[&'h'], vec![0]);
    }

    #[test]
    fn group_p() {
        assert_eq!(load().group[&'p'], vec![1, 7]);
    }

    #[test]
    fn group_z() {
        assert_eq!(load().group[&'z'], vec![8, 9]);
    }

    #[test]
    fn rejects_non_toml_extension() {
        let result: Result<Editex> = import("notatoml.json");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_file() {
        let result: Result<Editex> = import("nonexistent.toml");
        assert!(result.is_err());
    }
}
