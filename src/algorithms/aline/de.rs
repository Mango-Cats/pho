//! aline::de
//!
//! Private deserialization plumbing for ALINE types. The `[sounds.x]`
//! TOML header format requires an explicit `type` tag that has no
//! equivalent in the Rust type system, so raw intermediate types are
//! used to bridge the gap before converting into the canonical types
//! defined in `features.rs`.

use serde::{Deserialize, Deserializer};

use crate::algorithms::aline::phonemes::{ConsonantFeatures, PhoneticFeatures, VowelFeatures};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawSound {
    Consonant(ConsonantFeatures),
    Vowel(VowelFeatures),
}

impl<'de> Deserialize<'de> for PhoneticFeatures {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(match RawSound::deserialize(deserializer)? {
            RawSound::Consonant(c) => PhoneticFeatures::Consonant(c),
            RawSound::Vowel(v) => PhoneticFeatures::Vowel(v),
        })
    }
}
