//! # Algorithms
//!
//! String and phonetic similarity algorithms.
//!
//! Each algorithm struct implements the [`Algorithm`] trait, allowing uniform
//! scoring across the library. Algorithms can also be combined into a weighted
//! ensemble using [`crate::ensemble`].
//!
//! ## Usage
//!
//! Import an algorithm, load its configuration from TOML if needed, and call
//! `similarity(a, b)` on the instance.
//!

pub mod aline;
pub mod bisim;
pub mod double_metaphone;
pub mod editex;
pub mod jaro_winkler;
pub mod keyboard;
pub mod lcs;
pub mod lcsubstring;
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
pub mod visual;

pub use aline::config::Aline;
pub use bisim::config::BiSim;
pub use double_metaphone::config::DoubleMetaphone;
pub use editex::config::Editex;
pub use jaro_winkler::config::JaroWinkler;
pub use keyboard::config::Keyboard;
pub use lcs::LCS;
pub use lcsubstring::LCSubstring;
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
pub use visual::config::VisualWeighted;
