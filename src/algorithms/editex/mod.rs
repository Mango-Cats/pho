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
//! use pho::{algorithms::editex, config_io::read};
//! use pho::algorithms::editex::config::EditexConfig;
//!
//! let config: EditexConfig = read("tests/config_sample_editex.toml").unwrap();
//! let distance = editex::distance("Smith", "Smyth", &config).unwrap();
//! let similarity = editex::similarity("Smith", "Smyth", &config).unwrap();
//! assert!(distance >= 0.0);
//! assert!((0.0..=1.0).contains(&similarity));
//! ```

pub mod config;
pub mod edit;
pub mod group;

mod distance;
mod similarity;
mod tokenize;

pub use similarity::{distance, similarity};
