use serde::{Deserialize, Serialize};

use crate::{Error, Result, utils::validate::validated_serde};

/// Salience weights of the phonetic features used by ALINE.
///
/// Every weight must be finite and non-negative.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct Salience {
    pub(crate) syllabic: f32,
    pub(crate) place: f32,
    pub(crate) manner: f32,
    pub(crate) voice: f32,
    pub(crate) nasal: f32,
    pub(crate) retroflex: f32,
    pub(crate) lateral: f32,
    pub(crate) aspirated: f32,
    pub(crate) long: f32,
    pub(crate) high: f32,
    pub(crate) back: f32,
    pub(crate) round: f32,
    pub(crate) phonation: f32,
    pub(crate) airstream: f32,
    pub(crate) secondary: f32,
    /// Weight applied to stress-level differences (MangoCats variant only).
    /// Defaults to 0.0, which makes the Kondrak variant behave identically.
    #[serde(default)]
    pub(crate) stress: f32,
}

validated_serde!(Salience);

impl Salience {
    pub fn validate(&self) -> Result<()> {
        for (name, value) in [
            ("syllabic", self.syllabic),
            ("place", self.place),
            ("manner", self.manner),
            ("voice", self.voice),
            ("nasal", self.nasal),
            ("retroflex", self.retroflex),
            ("lateral", self.lateral),
            ("aspirated", self.aspirated),
            ("long", self.long),
            ("high", self.high),
            ("back", self.back),
            ("round", self.round),
            ("phonation", self.phonation),
            ("airstream", self.airstream),
            ("secondary", self.secondary),
            ("stress", self.stress),
        ] {
            if !(value.is_finite() && value >= 0.0) {
                return Err(Error::InvalidAlineSalience { name, value });
            }
        }
        Ok(())
    }

    pub fn try_new(
        syllabic: f32,
        place: f32,
        manner: f32,
        voice: f32,
        nasal: f32,
        retroflex: f32,
        lateral: f32,
        aspirated: f32,
        long: f32,
        high: f32,
        back: f32,
        round: f32,
        phonation: f32,
        airstream: f32,
        secondary: f32,
        stress: f32,
    ) -> Result<Self> {
        let salience = Self {
            syllabic,
            place,
            manner,
            voice,
            nasal,
            retroflex,
            lateral,
            aspirated,
            long,
            high,
            back,
            round,
            phonation,
            airstream,
            secondary,
            stress,
        };
        salience.validate()?;
        Ok(salience)
    }

    /// Salience weight of the `syllabic` feature.
    pub fn syllabic(&self) -> f32 {
        self.syllabic
    }

    /// Salience weight of the `place` feature.
    pub fn place(&self) -> f32 {
        self.place
    }

    /// Salience weight of the `manner` feature.
    pub fn manner(&self) -> f32 {
        self.manner
    }

    /// Salience weight of the `voice` feature.
    pub fn voice(&self) -> f32 {
        self.voice
    }

    /// Salience weight of the `nasal` feature.
    pub fn nasal(&self) -> f32 {
        self.nasal
    }

    /// Salience weight of the `retroflex` feature.
    pub fn retroflex(&self) -> f32 {
        self.retroflex
    }

    /// Salience weight of the `lateral` feature.
    pub fn lateral(&self) -> f32 {
        self.lateral
    }

    /// Salience weight of the `aspirated` feature.
    pub fn aspirated(&self) -> f32 {
        self.aspirated
    }

    /// Salience weight of the `long` feature.
    pub fn long(&self) -> f32 {
        self.long
    }

    /// Salience weight of the `high` feature.
    pub fn high(&self) -> f32 {
        self.high
    }

    /// Salience weight of the `back` feature.
    pub fn back(&self) -> f32 {
        self.back
    }

    /// Salience weight of the `round` feature.
    pub fn round(&self) -> f32 {
        self.round
    }

    /// Salience weight of the `phonation` feature.
    pub fn phonation(&self) -> f32 {
        self.phonation
    }

    /// Salience weight of the `airstream` feature.
    pub fn airstream(&self) -> f32 {
        self.airstream
    }

    /// Salience weight of the `secondary` feature.
    pub fn secondary(&self) -> f32 {
        self.secondary
    }

    /// Salience weight of the `stress` feature.
    pub fn stress(&self) -> f32 {
        self.stress
    }
}
