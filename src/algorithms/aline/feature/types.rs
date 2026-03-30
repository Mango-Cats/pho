/// This struct stores the configuration to be used by an Aline algorithm.
pub struct PhoneticConfig {
    pub place: PlaceValues,
    pub manner: MannerValues,
    pub high: HighValues,
    pub back: BackValues,
}

impl Default for PhoneticConfig {
    fn default() -> Self {
        Self {
            place: PlaceValues::default(),
            manner: MannerValues::default(),
            high: HighValues::default(),
            back: BackValues::default(),
        }
    }
}

/// This enum stores the types of features stored for each phonetic symbol
pub enum FeatureType {
    ConsonantFeatures,
    VowelFeatures,
}
/// Has the value of either Plus (+) or Minus (-). Plus denotes the
/// presence of a feature, while minus denotes the absence. For instance,
/// if the feature `aspirated` has the minus value, so that feature is
/// not aspirated.
pub enum Binary {
    Plus,
    Minus,
}

/// Are the possible places of articulation of a specific sound.
///
/// ## Places of Articulation
///
/// - Bilabial: Both lips are used.
/// - Labiodental: Lower lip touches the upper teeth.
/// - Dental: Tongue tip or blade touches the upper teeth.
/// - Alveolar: Tongue touches the gum ridge behind the upper teeth.
/// - PalatoAlveolar: Tongue near or touching the back of the alveolar ridge and the hard palate
/// - Palatal: Tongue touches the hard palate at the roof of the mouth.
/// - Velar: Tongue touches the soft palate.
/// - Uvular: Tongue touches or approaches the uvula.
/// - Pharyngeal: Produced in the pharynx.
/// - Glottal: Produced at the vocal folds/glottis.
pub enum Place {
    Bilabial,
    Labiodental,
    Dental,
    Alveolar,
    Retroflex,
    PalatoAlveolar,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Glottal,
    Labiovelar, // TODO: no code documentation for this
    Vowel,
}

/// The values of the place feature. The values are within the
/// range of [0,1] with 0 denoting the place of the sound is at the back
/// of the mouth (hence glottal is 0) and 1 denoting the front (hence
/// bilabial is 1).
pub struct PlaceValues {
    pub bilabial: f32,
    pub labiodental: f32,
    pub dental: f32,
    pub alveolar: f32,
    pub retroflex: f32,
    pub palato_alveolar: f32,
    pub palatal: f32,
    pub velar: f32,
    pub uvular: f32,
    pub pharyngeal: f32,
    pub glottal: f32,
    pub labiovelar: f32,
    pub vowel: f32,
}

impl Default for PlaceValues {
    fn default() -> Self {
        Self {
            bilabial: 1.0,
            labiovelar: 1.0,
            labiodental: 0.95,
            dental: 0.9,
            alveolar: 0.85,
            retroflex: 0.8,
            palato_alveolar: 0.75,
            palatal: 0.7,
            velar: 0.6,
            uvular: 0.5,
            pharyngeal: 0.3,
            glottal: 0.1,
            vowel: -1.0,
        }
    }
}

impl Place {
    pub fn value(&self, scores: &PlaceValues) -> f32 {
        match self {
            Place::Bilabial => scores.bilabial,
            Place::Labiodental => scores.labiodental,
            Place::Dental => scores.dental,
            Place::Alveolar => scores.alveolar,
            Place::Retroflex => scores.retroflex,
            Place::PalatoAlveolar => scores.palato_alveolar,
            Place::Palatal => scores.palatal,
            Place::Velar => scores.velar,
            Place::Uvular => scores.uvular,
            Place::Pharyngeal => scores.pharyngeal,
            Place::Glottal => scores.glottal,
            Place::Labiovelar => scores.labiovelar,
            Place::Vowel => scores.vowel,
        }
    }
}

/// Represents the manner of articulation or degree of stricture for a specific sound.
///
/// ## Manners of Articulation
///
/// - Stop: Complete blockage of airflow followed by a sudden release.
/// - Affricate: Begins as a stop and releases directly into a fricative.
/// - Fricative: Narrow constriction causing turbulent, noisy airflow.
/// - Trill: An articulator vibrates continuously against another due to airflow.
/// - Tap: A single, rapid strike of one articulator against another.
/// - Approximant: Articulators approach each other without creating turbulent airflow.
/// - High: Vowel produced with the tongue positioned close to the roof of the mouth.
/// - Mid: Vowel produced with the tongue midway between high and low positions.
/// - Low: Vowel produced with the tongue low and the mouth relatively open.
/// - Vowel: A generic vowel representation characterized by an open vocal tract.
pub enum Manner {
    Stop,
    Affricate,
    Fricative,
    Trill,
    Tap,
    Approximant,
    High,
    Mid,
    Low,
    Vowel,
}

/// The values of the manner feature. The values are within the
/// range of [0, 1] with 0 denoting minimal stricture or maximum openness
/// of the vocal tract (hence a low vowel is 0.0) and 1 denoting complete
/// blockage of airflow (hence a stop is 1.0).
pub struct MannerValues {
    pub stop: f32,
    pub affricate: f32,
    pub fricative: f32,
    pub trill: f32,
    pub tap: f32,
    pub approximant: f32,
    pub high: f32,
    pub mid: f32,
    pub low: f32,
    pub vowel: f32,
}

impl Default for MannerValues {
    fn default() -> Self {
        Self {
            stop: 1.0,
            affricate: 0.9,
            fricative: 0.85,
            trill: 0.7,
            tap: 0.65,
            approximant: 0.6,
            high: 0.4,
            mid: 0.2,
            low: 0.0,
            vowel: 0.5,
        }
    }
}

impl Manner {
    pub fn value(&self, scores: &MannerValues) -> f32 {
        match self {
            Manner::Stop => scores.stop,
            Manner::Affricate => scores.affricate,
            Manner::Fricative => scores.fricative,
            Manner::Trill => scores.trill,
            Manner::Tap => scores.tap,
            Manner::Approximant => scores.approximant,
            Manner::High => scores.high,
            Manner::Mid => scores.mid,
            Manner::Low => scores.low,
            Manner::Vowel => scores.vowel,
        }
    }
}

pub enum High {
    High,
    Mid,
    Low,
}

pub struct HighValues {
    pub high: f32,
    pub mid: f32,
    pub low: f32,
}

impl Default for HighValues {
    fn default() -> Self {
        Self {
            high: 1.0,
            mid: 0.5,
            low: 0.0,
        }
    }
}

impl High {
    pub fn value(&self, scores: &HighValues) -> f32 {
        match self {
            High::High => scores.high,
            High::Mid => scores.mid,
            High::Low => scores.low,
        }
    }
}

pub enum Back {
    Front,
    Central,
    Back,
}

pub struct BackValues {
    pub front: f32,
    pub central: f32,
    pub back: f32,
}

impl Default for BackValues {
    fn default() -> Self {
        Self {
            front: 1.0,
            central: 0.5,
            back: 0.0,
        }
    }
}

impl Back {
    pub fn value(&self, scores: &BackValues) -> f32 {
        match self {
            Back::Front => scores.front,
            Back::Central => scores.central,
            Back::Back => scores.back,
        }
    }
}

/// This struct stores the features stored by consonants.
///
/// ## Features
///
/// - aspirated: with breath.
/// - lateral: the tongue is touching the roof of the mouth.
/// - manner
pub struct ConsonantFeatures {
    pub aspirated: Binary,
    pub lateral: Binary,
    pub manner: Manner,
    pub nasal: Binary,
    pub place: Place,
    pub retroflex: Binary,
    pub syllabic: Binary,
    pub voice: Binary,
}

pub struct VowelFeatures {
    pub back: Back,
    pub high: High,
    pub lateral: Binary,
    pub long: Binary,
    pub nasal: Binary,
    pub retroflex: Binary,
    pub round: Binary,
    pub syllabic: Binary,
    pub voice: Binary,
}
