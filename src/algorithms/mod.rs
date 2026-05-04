//! # Algorithms
//!
//! This module contains the source code for the similarity algorithms.
//!
//! ## Algorithms
//!
//! - [aline]
//! - [editex].
//! - [jaro_winkler]
//! - [levenshtein]
//! - [ngram]
//!
//! ## Polymorphic Algorithms
//!
//! The direct algorithm structs implement [`Algorithm`], so they can
//! be used uniformly without a separate wrapper layer.
//!
//! This also allows multiple algorithms to be combined into a single
//! ensemble, see [ensemble].
//!
//! ## Usage
//!
//! The top-level module documentation gives an example of each
//! algorithm's use. In general, you import the direct algorithm type,
//! deserialize it from TOML if needed, and then call `similarity(a, b)` on
//! the resulting value.
//!

pub mod aline;
pub mod editex;
pub mod jaro_winkler;
pub mod lcs;
pub mod lcsuf;
pub mod levenshtein;
pub mod ngram;
mod traits;

pub use aline::config::Aline;
pub use editex::config::Editex;
pub use jaro_winkler::config::JaroWinkler;
pub use lcs::LCS;
pub use lcsuf::LCSuf;
pub use levenshtein::config::Levenshtein;
pub use ngram::config::{NGram, NGramMetric};
pub use traits::Algorithm;
