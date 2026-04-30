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
/// Aline uses four constants for reward or penalty; a negative value
/// denotes a penalty while a positive denotes a reward.
///
/// 1. `skip` is the constant for an indel (insert or delete).
///
/// 2. `subtitute` is the constant for a substitution (when one phoneme
/// is replaced with another).
///
/// 3. `expand_compress` is the constant for when a phoneme matches two
/// phonemes in another. Example: "suit" can be pronounced as [sut] and
/// [suwt], so the /u/ sound is expanded to the /uw/ sound.
///
/// 4. `vowel_consonant` is the relative weight for vowels versus
/// consonants.
///
/// ## References
///
/// - https://dl.acm.org/doi/book/10.5555/936774
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Costs {
    pub skip: i32,
    pub substitute: i32,
    pub expand_compress: i32,
    pub vowel_consonant: i32,
}
