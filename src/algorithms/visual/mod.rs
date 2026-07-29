//! visual
//!
//! A visual-confusability weighted edit distance for Latin letters.
//!
//! ## What this computes
//!
//! - `distance(a, b)` computes a weighted edit distance between two strings,
//!   structurally identical to [`crate::algorithms::Editex`] but driven by
//!   *visual* letter-shape groups instead of phonetic ones (e.g. `b`/`d`,
//!   `i`/`j`/`l`, `v`/`w`/`y`).
//! - `similarity(a, b)` computes a normalized score in $[0, 1]$:
//!   $$\text{similarity}(a,b) = 1 - \frac{\text{distance}(a,b)}{\text{max\_distance}(a,b)}$$
//!
//! Groups are derived from Simpson, Mousikou, Montoya & Defior (2013), "A
//! letter visual-similarity matrix for Latin-based alphabets" (Behavior
//! Research Methods), restricted to the letters that cluster together once
//! accented and font-variant forms are dropped (ASCII a-z / A-Z only). The
//! source study did not rate cross-case pairs, so uppercase and lowercase
//! groups never overlap.
//!
//! By default (`case_insensitive = false`) both cases are compared as
//! written, using their own independent group table. Set
//! `case_insensitive = true` to casefold both inputs first, in which case
//! only the lowercase groups are consulted.
//!
//! ## Example
//!
//! ```rust
//! use pho::{algorithms::{VisualWeighted, Algorithm}, utils::io::import};
//!
//! let algo: VisualWeighted = import("algorithm_configs/eng/visual_weighted.toml").unwrap();
//! let similarity = algo.similarity("modern", "rnodern").unwrap();
//! assert!((0.0..=1.0).contains(&similarity));
//! ```
//!
//! ## References
//!
//! - Simpson, I. C., Mousikou, P., Montoya, J. M., & Defior, S. (2013). A
//!   letter visual-similarity matrix for Latin-based alphabets. *Behavior
//!   Research Methods*, 45(2), 431-439.

pub mod config;
pub mod edit;
pub mod group;

mod distance;

use crate::{algorithms::Algorithm, error::Result, utils::validate::validate_tokens};

use config::VisualWeighted;
use distance::{distance, operation_counts, total_delete_cost};

fn normalized_chars(
    input: &str,
    input_name: &'static str,
    config: &VisualWeighted,
) -> Result<Vec<char>> {
    let chars = input.chars().map(|c| {
        if config.case_insensitive {
            c.to_ascii_lowercase()
        } else {
            c
        }
    });

    validate_tokens(
        chars.filter(|c| c.is_ascii_alphabetic()),
        input_name,
        "Visual weighted config groups",
        |symbol| config.group.contains_key(symbol),
    )
}

impl Algorithm for VisualWeighted {
    fn distance(&self, x: &str, y: &str) -> Result<f32> {
        let x_chars = normalized_chars(x, "x", self)?;
        let y_chars = normalized_chars(y, "y", self)?;

        Ok(distance(&x_chars, &y_chars, self))
    }

    fn normalized_distance(&self, x: &str, y: &str) -> Result<f32> {
        let x_chars = normalized_chars(x, "x", self)?;
        let y_chars = normalized_chars(y, "y", self)?;

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

    fn edit_operation_counts(&self, x: &str, y: &str) -> Result<(u32, u32, u32)> {
        let x_chars = normalized_chars(x, "x", self)?;
        let y_chars = normalized_chars(y, "y", self)?;

        Ok(operation_counts(&x_chars, &y_chars, self))
    }

    fn separate_enabled(&self) -> bool {
        self.separate
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        algorithms::{Algorithm, VisualWeighted},
        error::Result,
        utils::io::import,
    };

    const TOML_PATH: &str = "algorithm_configs/eng/visual_weighted.toml";

    fn load() -> VisualWeighted {
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
    fn lowercase_bd_share_a_group() {
        let config = load();
        let b = &config.group[&'b'];
        let d = &config.group[&'d'];
        assert!(b.iter().any(|g| d.contains(g)));
    }

    #[test]
    fn lowercase_ijl_share_a_group() {
        let config = load();
        let i = &config.group[&'i'];
        let j = &config.group[&'j'];
        let l = &config.group[&'l'];
        assert!(i.iter().any(|g| j.contains(g)));
        assert!(i.iter().any(|g| l.contains(g)));
    }

    #[test]
    fn lowercase_unclustered_letter_has_no_groups() {
        assert!(load().group[&'a'].is_empty());
    }

    #[test]
    fn uppercase_and_lowercase_groups_do_not_overlap() {
        let config = load();
        let upper_b = &config.group[&'B'];
        let lower_b = &config.group[&'b'];
        assert!(!upper_b.iter().any(|g| lower_b.contains(g)));
    }

    #[test]
    fn similarity_is_bounded() {
        let config = load();
        let score = config.similarity("modern", "rnodern").unwrap();
        assert!((0.0..=1.0).contains(&score));
    }

    #[test]
    fn confusable_pair_scores_higher_than_unrelated_pair() {
        let config = load();
        let confusable = config.similarity("bat", "dat").unwrap();
        let unrelated = config.similarity("bat", "zat").unwrap();
        assert!(confusable > unrelated);
    }

    #[test]
    fn identical_strings_score_one() {
        let config = load();
        assert_eq!(config.similarity("bad", "bad").unwrap(), 1.0);
    }

    #[test]
    fn rejects_non_toml_extension() {
        let result: Result<VisualWeighted> = import("notatoml.json");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_file() {
        let result: Result<VisualWeighted> = import("nonexistent.toml");
        assert!(result.is_err());
    }
}
