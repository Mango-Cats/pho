//! aline::config::cost
//!
//! This file contains the `Costs` struct used by the Aline algorithm.
//! This struct contains the constants that are used for the similarity
//! reward or penalty.

use serde::{Deserialize, Serialize};

/// This struct holds the cost constants for the Aline algorithm.
///
/// ## Cost Variables
///
/// Aline uses four constants for rewards and penalties. Negative values
/// denote penalties while positive values denote rewards.
///
/// 1. `skip` is the constant for an indel (insert or delete).
///
/// 2. `substitute` is the constant for replacing one phoneme with another.
///
/// 3. `expand_compress` is the constant for when one phoneme matches two
/// phonemes in another (for example, expanding an /u/ sound into /uw/).
///
/// 4. `vowel_consonant` is the relative weight for vowels versus consonants.
///
/// ## References
///
/// - <https://dl.acm.org/doi/book/10.5555/936774>
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Costs {
    pub(crate) skip: i32,
    pub(crate) substitute: i32,
    pub(crate) expand_compress: i32,
    pub(crate) vowel_consonant: i32,
}

impl Costs {
    pub fn new(skip: i32, substitute: i32, expand_compress: i32, vowel_consonant: i32) -> Self {
        Self {
            skip,
            substitute,
            expand_compress,
            vowel_consonant,
        }
    }
}
