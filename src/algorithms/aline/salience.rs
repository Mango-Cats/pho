use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Salience {
    pub syllabic: u32,
    pub place: u32,
    pub manner: u32,
    pub voice: u32,
    pub nasal: u32,
    pub retroflex: u32,
    pub lateral: u32,
    pub aspirated: u32,
    pub long: u32,
    pub high: u32,
    pub back: u32,
    pub round: u32,
}
