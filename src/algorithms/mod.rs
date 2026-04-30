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
//! In the event this is not updated, refer to [registry].
//!
//! ## Polymorphic Algorithms
//!
//! Refer to the following: [traits], [wrapper], and [dispatcher].
//! These three allow for all algorithms to be polymorphic.
//!
//! This allows us to combine multiple algorithms into a single
//! combined algorithm, see [ensemble].
//!
//! ## Usage
//!
//! The top-level module documentation gives an example of each
//! algorithms use. In general, you need to define a config first (if
//! the algorithm requires a config). Then, for any algorithm `X` and
//! two strings `a` and `b`, calling `X::similarity(a,b, &config)`
//! will return a `Result` type that will contain a score (a float in
//! the range of [0,1]) or an error.
//!

pub mod aline;
pub mod editex;
pub mod ensemble;
pub mod jaro_winkler;
pub mod levenshtein;

mod dispatcher;
mod errors;
mod registry;
mod traits;
mod wrapper;

pub use errors::UnknownTokenError;
pub use registry::{Algorithm, AlgorithmConfig};
pub use traits::AlgorithmTrait;
pub use wrapper::{AlineAlgorithm, EditexAlgorithm, JaroWinklerAlgorithm, LevenshteinAlgorithm};
