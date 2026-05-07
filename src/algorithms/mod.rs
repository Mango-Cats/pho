//! # Algorithms
//!
//! This module contains the source code for the similarity algorithms.
//!
//! ## Algorithms
//!
//! - [aline]
//! - [editex]
//! - [jaro_winkler]
//! - [bisim]
//! - [prefix]
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
pub mod bisim;
pub mod double_metaphone;
pub mod editex;
pub mod jaro_winkler;
pub mod keyboard;
pub mod lcs;
pub mod lcsuf;
pub mod levenshtein;
pub mod metaphone;
pub mod needleman_wunsch;
pub mod ngram;
pub mod prefix;
pub mod smith_waterman;
pub mod soundex;
pub mod syllable;
pub mod tfidf;
mod traits;

pub use aline::config::Aline;
pub use bisim::config::BiSim;
pub use double_metaphone::config::DoubleMetaphone;
pub use editex::config::Editex;
pub use jaro_winkler::config::JaroWinkler;
pub use keyboard::config::Keyboard;
pub use lcs::LCS;
pub use lcsuf::LCSuf;
pub use levenshtein::config::Levenshtein;
pub use metaphone::config::Metaphone;
pub use needleman_wunsch::config::NeedlemanWunsch;
pub use ngram::{config::NGram, metric::NGramMetric};
pub use prefix::config::Prefix;
pub use smith_waterman::config::SmithWaterman;
pub use soundex::config::Soundex;
pub use syllable::config::Syllable;
pub use tfidf::config::CharTfIdf;
pub use traits::Algorithm;
