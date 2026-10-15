//! aline::config::cost
//!
//! This file contains the `Costs` struct used by the Aline algorithm.
//! This struct contains the constants that are used for the similarity
//! reward or penalty.

use serde::{Deserialize, Serialize};

use crate::{Error, Result, utils::validate::validated_serde};

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
/// `skip` must be non-positive; the other three must be non-negative.
///
/// ## References
///
/// - <https://dl.acm.org/doi/book/10.5555/936774>
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct Costs {
    pub(crate) skip: i32,
    pub(crate) substitute: i32,
    pub(crate) expand_compress: i32,
    pub(crate) vowel_consonant: i32,
}

validated_serde!(Costs);

impl Costs {
    pub fn validate(&self) -> Result<()> {
        if self.skip > 0 {
            return Err(Error::InvalidAlineCost {
                name: "skip",
                expected: "non-positive",
                value: self.skip,
            });
        }
        for (name, value) in [
            ("substitute", self.substitute),
            ("expand_compress", self.expand_compress),
            ("vowel_consonant", self.vowel_consonant),
        ] {
            if value < 0 {
                return Err(Error::InvalidAlineCost {
                    name,
                    expected: "non-negative",
                    value,
                });
            }
        }
        Ok(())
    }

    pub fn try_new(
        skip: i32,
        substitute: i32,
        expand_compress: i32,
        vowel_consonant: i32,
    ) -> Result<Self> {
        let costs = Self {
            skip,
            substitute,
            expand_compress,
            vowel_consonant,
        };
        costs.validate()?;
        Ok(costs)
    }

    /// The `skip` cost.
    pub fn skip(&self) -> i32 {
        self.skip
    }

    /// The `substitute` cost.
    pub fn substitute(&self) -> i32 {
        self.substitute
    }

    /// The `expand_compress` cost.
    pub fn expand_compress(&self) -> i32 {
        self.expand_compress
    }

    /// The `vowel_consonant` cost.
    pub fn vowel_consonant(&self) -> i32 {
        self.vowel_consonant
    }
}
