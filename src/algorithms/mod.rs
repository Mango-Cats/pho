//! # Algorithms
//!
//! This module contains the source code for the similarity algorithms.
//!
//! ## Algorithms
//!
//! - [aline]. A New Algorithm for the Alignment of Phonetic
//!     Sequences (Kondrak, ANLP 2000).
//! - [editex]. Phonetic string matching: lessons from information
//!     retrieval (Zobel and Dart, SIGIR 1996).
//! - [jaro_winkler].
//! - [levenshtein].
//!
//! In the event this is not updated, refer to the module docs.
//!
//! ## Polymorphic Algorithms
//!
//! The direct algorithm structs implement [`AlgorithmTrait`], so they can
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
pub mod ensemble;
pub mod jaro_winkler;
pub mod levenshtein;

mod traits;

pub use aline::config::Aline;
pub use editex::config::Editex;
pub use jaro_winkler::config::JaroWinkler;
pub use levenshtein::config::Levenshtein;
pub use traits::AlgorithmTrait;
