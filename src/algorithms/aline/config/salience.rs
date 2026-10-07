use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Salience {
    pub(crate) syllabic: u32,
    pub(crate) place: u32,
    pub(crate) manner: u32,
    pub(crate) voice: u32,
    pub(crate) nasal: u32,
    pub(crate) retroflex: u32,
    pub(crate) lateral: u32,
    pub(crate) aspirated: u32,
    pub(crate) long: u32,
    pub(crate) high: u32,
    pub(crate) back: u32,
    pub(crate) round: u32,
    pub(crate) phonation: u32,
    pub(crate) airstream: u32,
    pub(crate) secondary: u32,
    /// Weight applied to stress-level differences (MangoCats variant only).
    /// Defaults to 0, which makes the Kondrak variant behave identically.
    #[serde(default)]
    pub(crate) stress: u32,
}

impl Salience {
    pub fn new(
        syllabic: u32,
        place: u32,
        manner: u32,
        voice: u32,
        nasal: u32,
        retroflex: u32,
        lateral: u32,
        aspirated: u32,
        long: u32,
        high: u32,
        back: u32,
        round: u32,
        phonation: u32,
        airstream: u32,
        secondary: u32,
        stress: u32,
    ) -> Self {
        Self {
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
        }
    }

    /// Salience weight of the `syllabic` feature.
    pub fn syllabic(&self) -> u32 {
        self.syllabic
    }

    /// Salience weight of the `place` feature.
    pub fn place(&self) -> u32 {
        self.place
    }

    /// Salience weight of the `manner` feature.
    pub fn manner(&self) -> u32 {
        self.manner
    }

    /// Salience weight of the `voice` feature.
    pub fn voice(&self) -> u32 {
        self.voice
    }

    /// Salience weight of the `nasal` feature.
    pub fn nasal(&self) -> u32 {
        self.nasal
    }

    /// Salience weight of the `retroflex` feature.
    pub fn retroflex(&self) -> u32 {
        self.retroflex
    }

    /// Salience weight of the `lateral` feature.
    pub fn lateral(&self) -> u32 {
        self.lateral
    }

    /// Salience weight of the `aspirated` feature.
    pub fn aspirated(&self) -> u32 {
        self.aspirated
    }

    /// Salience weight of the `long` feature.
    pub fn long(&self) -> u32 {
        self.long
    }

    /// Salience weight of the `high` feature.
    pub fn high(&self) -> u32 {
        self.high
    }

    /// Salience weight of the `back` feature.
    pub fn back(&self) -> u32 {
        self.back
    }

    /// Salience weight of the `round` feature.
    pub fn round(&self) -> u32 {
        self.round
    }

    /// Salience weight of the `phonation` feature.
    pub fn phonation(&self) -> u32 {
        self.phonation
    }

    /// Salience weight of the `airstream` feature.
    pub fn airstream(&self) -> u32 {
        self.airstream
    }

    /// Salience weight of the `secondary` feature.
    pub fn secondary(&self) -> u32 {
        self.secondary
    }

    /// Salience weight of the `stress` feature.
    pub fn stress(&self) -> u32 {
        self.stress
    }
}
