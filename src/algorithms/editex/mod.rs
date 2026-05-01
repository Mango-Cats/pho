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
//! use pho::{algorithms::{AlgorithmTrait, EditexAlgorithm}, config_io::import};
//! use pho::algorithms::editex::config::EditexConfig;
//!
//! let config: EditexConfig = import("tests/config_sample_editex.toml").unwrap();
//! let algo = EditexAlgorithm::new(&config);
//! let similarity = algo.similarity("Smith", "Smyth").unwrap();
//! assert!((0.0..=1.0).contains(&similarity));
//! ```

pub mod config;
pub mod edit;
pub mod group;

mod distance;
mod similarity;
mod tokenize;

pub(crate) use similarity::similarity;
