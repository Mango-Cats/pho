use crate::algorithms::aline::{
    cost::Costs,
    features::{
        Back, Binary, ConsonantFeatures, FeatureValues, High, Manner, PhoneticFeatures, Place,
        VowelFeatures,
    },
    salience::Salience,
};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct AlineConfig {
    pub costs: Costs,
    pub salience: Salience,
    pub values: FeatureValues,
    pub sounds: HashMap<String, PhoneticFeatures>,
}

#[derive(Deserialize)]
struct RawConsonant {
    place: Place,
    manner: Manner,
    syllabic: Binary,
    voice: Binary,
    nasal: Binary,
    retroflex: Binary,
    lateral: Binary,
    aspirated: Binary,
}

#[derive(Deserialize)]
struct RawVowel {
    place: Place,
    manner: Manner,
    back: Back,
    high: High,
    lateral: Binary,
    long: Binary,
    nasal: Binary,
    retroflex: Binary,
    round: Binary,
    syllabic: Binary,
    voice: Binary,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawSound {
    Consonant(RawConsonant),
    Vowel(RawVowel),
}

impl From<RawSound> for PhoneticFeatures {
    fn from(raw: RawSound) -> Self {
        match raw {
            RawSound::Consonant(c) => PhoneticFeatures::Consonant(ConsonantFeatures {
                place: c.place,
                manner: c.manner,
                syllabic: c.syllabic,
                voice: c.voice,
                nasal: c.nasal,
                retroflex: c.retroflex,
                lateral: c.lateral,
                aspirated: c.aspirated,
            }),
            RawSound::Vowel(v) => PhoneticFeatures::Vowel(VowelFeatures {
                place: v.place,
                manner: v.manner,
                back: v.back,
                high: v.high,
                lateral: v.lateral,
                long: v.long,
                nasal: v.nasal,
                retroflex: v.retroflex,
                round: v.round,
                syllabic: v.syllabic,
                voice: v.voice,
            }),
        }
    }
}

impl<'de> Deserialize<'de> for PhoneticFeatures {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        RawSound::deserialize(deserializer).map(Into::into)
    }
}
