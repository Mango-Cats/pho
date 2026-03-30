use crate::algorithms::aline::salience::Salience;

/// Has the value of either Plus (+) or Minus (-). Plus denotes the
/// presence of a feature, while minus denotes the absence. For instance,
/// if the feature `aspirated` has the minus value, so that feature is
/// not aspirated.
#[derive(Debug, Clone, PartialEq)]
pub enum Binary {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub struct BinaryValues {
    pub plus: f32,
    pub minus: f32,
}

impl Binary {
    pub fn value(&self, values: &BinaryValues) -> f32 {
        match self {
            Binary::Plus => values.plus,
            Binary::Minus => values.minus,
        }
    }
}

impl Default for BinaryValues {
    fn default() -> Self {
        Self {
            plus: 1.0,
            minus: 0.0,
        }
    }
}

/// Are the possible places of articulation of a specific sound.
///
/// ## Places of Articulation
///
/// - Bilabial: Both lips are used.
/// - Labiodental: Lower lip touches the upper teeth.
/// - Dental: Tongue tip or blade touches the upper teeth.
/// - Alveolar: Tongue touches the gum ridge behind the upper teeth.
/// - PalatoAlveolar: Tongue near or touching the back of the alveolar ridge and the hard palate.
/// - Palatal: Tongue touches the hard palate at the roof of the mouth.
/// - Velar: Tongue touches the soft palate.
/// - Uvular: Tongue touches or approaches the uvula.
/// - Pharyngeal: Produced in the pharynx.
/// - Glottal: Produced at the vocal folds/glottis.
/// - Labiovelar: Simultaneous bilabial and velar articulation.
#[derive(Debug, Clone, PartialEq)]
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
    Labiovelar,
    Vowel,
}

impl Place {
    pub fn value(&self, values: &PlaceValues) -> f32 {
        match self {
            Place::Bilabial => values.bilabial,
            Place::Labiodental => values.labiodental,
            Place::Dental => values.dental,
            Place::Alveolar => values.alveolar,
            Place::Retroflex => values.retroflex,
            Place::PalatoAlveolar => values.palato_alveolar,
            Place::Palatal => values.palatal,
            Place::Velar => values.velar,
            Place::Uvular => values.uvular,
            Place::Pharyngeal => values.pharyngeal,
            Place::Glottal => values.glottal,
            Place::Labiovelar => values.labiovelar,
            Place::Vowel => values.vowel,
        }
    }
}

/// The values of the place feature. The values are within the
/// range of [0,1] with 0 denoting the place of the sound is at the back
/// of the mouth (hence glottal is 0) and 1 denoting the front (hence
/// bilabial is 1).
#[derive(Debug, Clone)]
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
/// - HighVowel: Vowel produced with the tongue close to the roof of the mouth.
/// - MidVowel: Vowel produced with the tongue midway between high and low positions.
/// - LowVowel: Vowel produced with the tongue low and the mouth relatively open.
/// - Vowel: A generic vowel representation characterized by an open vocal tract.
#[derive(Debug, Clone, PartialEq)]
pub enum Manner {
    Stop,
    Affricate,
    Fricative,
    Trill,
    Tap,
    Approximant,
    HighVowel,
    MidVowel,
    LowVowel,
    Vowel,
}

impl Manner {
    pub fn value(&self, values: &MannerValues) -> f32 {
        match self {
            Manner::Stop => values.stop,
            Manner::Affricate => values.affricate,
            Manner::Fricative => values.fricative,
            Manner::Trill => values.trill,
            Manner::Tap => values.tap,
            Manner::Approximant => values.approximant,
            Manner::HighVowel => values.high_vowel,
            Manner::MidVowel => values.mid_vowel,
            Manner::LowVowel => values.low_vowel,
            Manner::Vowel => values.vowel,
        }
    }
}

/// The values of the manner feature. The values are within the
/// range of [0, 1] with 0 denoting minimal stricture or maximum openness
/// of the vocal tract (hence a low vowel is 0.0) and 1 denoting complete
/// blockage of airflow (hence a stop is 1.0).
#[derive(Debug, Clone)]
pub struct MannerValues {
    pub stop: f32,
    pub affricate: f32,
    pub fricative: f32,
    pub trill: f32,
    pub tap: f32,
    pub approximant: f32,
    pub high_vowel: f32,
    pub mid_vowel: f32,
    pub low_vowel: f32,
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
            high_vowel: 0.4,
            mid_vowel: 0.2,
            low_vowel: 0.0,
            vowel: 0.5,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum High {
    High,
    Mid,
    Low,
}

#[derive(Debug, Clone)]
pub struct HighValues {
    pub high: f32,
    pub mid: f32,
    pub low: f32,
}

impl High {
    pub fn value(&self, values: &HighValues) -> f32 {
        match self {
            High::High => values.high,
            High::Mid => values.mid,
            High::Low => values.low,
        }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub enum Back {
    Front,
    Central,
    Back,
}

impl Back {
    pub fn value(&self, values: &BackValues) -> f32 {
        match self {
            Back::Front => values.front,
            Back::Central => values.central,
            Back::Back => values.back,
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct FeatureValues {
    pub place: PlaceValues,
    pub manner: MannerValues,
    pub high: HighValues,
    pub back: BackValues,
    pub binary: BinaryValues,
}

impl Default for FeatureValues {
    fn default() -> Self {
        Self {
            place: PlaceValues::default(),
            manner: MannerValues::default(),
            high: HighValues::default(),
            back: BackValues::default(),
            binary: BinaryValues::default(),
        }
    }
}

/// Shared interface for consonants and vowels. The ALINE sigma function
/// compares any two sounds using whichever features they have in common,
/// so both types need to expose the shared features through a common interface.
pub trait Phoneme {
    fn place(&self) -> &Place;
    fn manner(&self) -> &Manner;
    fn syllabic(&self) -> &Binary;
    fn voice(&self) -> &Binary;
    fn nasal(&self) -> &Binary;
    fn retroflex(&self) -> &Binary;
    fn lateral(&self) -> &Binary;
    fn is_vowel(&self) -> bool;
}

/// Stores the phonetic features of a consonant sound.
///
/// ## Features
///
/// - `aspirated`: Whether the sound is produced with an audible puff of breath.
/// - `lateral`: Whether airflow passes around the sides of the tongue.
/// - `manner`: The degree of constriction in the vocal tract.
/// - `nasal`: Whether airflow passes through the nasal cavity.
/// - `place`: Where in the vocal tract the constriction occurs.
/// - `retroflex`: Whether the tongue tip curls back toward the palate.
/// - `syllabic`: Whether the sound can form the nucleus of a syllable.
/// - `voice`: Whether the vocal folds vibrate during production.
#[derive(Debug, Clone)]
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

impl Phoneme for ConsonantFeatures {
    fn place(&self) -> &Place {
        &self.place
    }
    fn manner(&self) -> &Manner {
        &self.manner
    }
    fn syllabic(&self) -> &Binary {
        &self.syllabic
    }
    fn voice(&self) -> &Binary {
        &self.voice
    }
    fn nasal(&self) -> &Binary {
        &self.nasal
    }
    fn retroflex(&self) -> &Binary {
        &self.retroflex
    }
    fn lateral(&self) -> &Binary {
        &self.lateral
    }
    fn is_vowel(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
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
    pub place: Place,
    pub manner: Manner,
}

impl Phoneme for VowelFeatures {
    fn place(&self) -> &Place {
        &self.place
    }
    fn manner(&self) -> &Manner {
        &self.manner
    }
    fn syllabic(&self) -> &Binary {
        &self.syllabic
    }
    fn voice(&self) -> &Binary {
        &self.voice
    }
    fn nasal(&self) -> &Binary {
        &self.nasal
    }
    fn retroflex(&self) -> &Binary {
        &self.retroflex
    }
    fn lateral(&self) -> &Binary {
        &self.lateral
    }
    fn is_vowel(&self) -> bool {
        true
    }
}

/// A product type over consonant and vowel features. Pattern match on
/// this when you need vowel-specific features; use `.as_phoneme()` when
/// you only need the shared interface.
#[derive(Debug, Clone)]
pub enum PhoneticFeatures {
    Consonant(ConsonantFeatures),
    Vowel(VowelFeatures),
}

impl PhoneticFeatures {
    /// Returns a reference to the shared `Phoneme` trait object so callers
    /// don't need to match on the variant just to read common features.
    pub fn as_phoneme(&self) -> &dyn Phoneme {
        match self {
            PhoneticFeatures::Consonant(c) => c,
            PhoneticFeatures::Vowel(v) => v,
        }
    }

    /// Computes the phonetic similarity between two sounds (the ALINE `sigma`
    /// function). Returns a score in the range [0, C_sub] where a higher score
    /// means the two sounds are more similar.
    ///
    /// The score is computed as:
    /// `sigma(p, q) = C_sub - diff(p, q)`
    ///
    /// where `diff` sums the salience-weighted absolute difference between
    /// each shared feature value.
    pub fn sigma(
        &self,
        other: &PhoneticFeatures,
        values: &FeatureValues,
        salience: &Salience,
        c_sub: i32,
    ) -> f32 {
        let p = self.as_phoneme();
        let q = other.as_phoneme();

        let diff = diff_feature(
            p.place().value(&values.place),
            q.place().value(&values.place),
            salience.place,
        ) + diff_feature(
            p.manner().value(&values.manner),
            q.manner().value(&values.manner),
            salience.manner,
        ) + diff_feature(
            p.syllabic().value(&values.binary),
            q.syllabic().value(&values.binary),
            salience.syllabic,
        ) + diff_feature(
            p.voice().value(&values.binary),
            q.voice().value(&values.binary),
            salience.voice,
        ) + diff_feature(
            p.nasal().value(&values.binary),
            q.nasal().value(&values.binary),
            salience.nasal,
        ) + diff_feature(
            p.retroflex().value(&values.binary),
            q.retroflex().value(&values.binary),
            salience.retroflex,
        ) + diff_feature(
            p.lateral().value(&values.binary),
            q.lateral().value(&values.binary),
            salience.lateral,
        );

        // vowel-only features
        // this only occures when both sounds are vowels
        let vowel_diff = match (self, other) {
            (PhoneticFeatures::Vowel(a), PhoneticFeatures::Vowel(b)) => {
                diff_feature(
                    a.high.value(&values.high),
                    b.high.value(&values.high),
                    salience.high,
                ) + diff_feature(
                    a.back.value(&values.back),
                    b.back.value(&values.back),
                    salience.back,
                ) + diff_feature(
                    a.round.value(&values.binary),
                    b.round.value(&values.binary),
                    salience.round,
                ) + diff_feature(
                    a.long.value(&values.binary),
                    b.long.value(&values.binary),
                    salience.long,
                )
            }
            _ => 0.0,
        };

        (c_sub as f32) - (diff + vowel_diff)
    }
}

/// Computes the salience-weighted absolute difference between two feature values.
/// This mirrors: `salience[f] * |similarity_matrix[p[f]] - similarity_matrix[q[f]]|`
#[inline]
fn diff_feature(a: f32, b: f32, salience: u32) -> f32 {
    salience as f32 * (a - b).abs()
}
