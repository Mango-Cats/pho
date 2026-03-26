pub enum FeatureType {
    ConsonantFeatures,
    VowelFeatures,
}

pub struct ConsonantFeatures {
    pub aspirated: i32,
    pub lateral: i32,
    pub manner: i32,
    pub nasal: i32,
    pub place: i32,
    pub retroflex: i32,
    pub syllabic: i32,
    pub voice: i32,
}

pub struct VowelFeatures {
    pub back: i32,
    pub lateral: i32,
    pub long: i32,
    pub manner: i32,
    pub nasal: i32,
    pub place: i32,
    pub retroflex: i32,
    pub round: i32,
    pub syllabic: i32,
    pub voice: i32,
}
