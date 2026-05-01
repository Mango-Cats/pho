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
        }
    }
}
