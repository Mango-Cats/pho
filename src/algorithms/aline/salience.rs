use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Salience {
    pub syllabic: u32,  // 5
    pub place: u32,     // 40
    pub manner: u32,    // 50
    pub voice: u32,     // 5
    pub nasal: u32,     // 20
    pub retroflex: u32, // 10
    pub lateral: u32,   // 10
    pub aspirated: u32, // 5
    pub long: u32,      // 0
    pub high: u32,      // 3
    pub back: u32,      // 2
    pub round: u32,     // 2
}

impl Default for Salience {
    fn default() -> Self {
        Self {
            syllabic: 5,
            place: 40,
            manner: 50,
            voice: 5,
            nasal: 20,
            retroflex: 10,
            lateral: 10,
            aspirated: 5,
            long: 0,
            high: 3,
            back: 2,
            round: 2,
        }
    }
}
