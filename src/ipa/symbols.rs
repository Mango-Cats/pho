//! IPA (International Phonetic Alphabet) symbols for representing sounds.

use std::fmt;

/// IPA phoneme symbols.
///
/// Organized by manner and place of articulation following IPA conventions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IPA {
    // VOWELS - Monophthongs
    /// /ɑ/ - Open back unrounded (father, spa)
    OpenBackUnrounded,
    /// /æ/ - Near-open front unrounded (cat, bat)  
    NearOpenFront,
    /// /ʌ/ - Open-mid back unrounded (strut, cup)
    OpenMidBack,
    /// /ə/ - Mid central (schwa) (about, sofa)
    Schwa,
    /// /ɔ/ - Open-mid back rounded (thought, law)
    OpenMidBackRounded,
    /// /ɛ/ - Open-mid front unrounded (bed, get)
    OpenMidFront,
    /// /ɜ/ - Open-mid central unrounded (girl, earth)
    OpenMidCentral,
    /// /ɝ/ - R-colored mid central stressed (bird, nurse)
    RColoredMid,
    /// /ɚ/ - R-colored schwa unstressed (butter, sister)
    RColoredSchwa,
    /// /ɪ/ - Near-close near-front unrounded (bit, kid)
    NearCloseFront,
    /// /i/ - Close front unrounded (fleece, see)
    CloseFront,
    /// /ʊ/ - Near-close near-back rounded (foot, put)
    NearCloseBack,
    /// /u/ - Close back rounded (goose, blue)
    CloseBack,

    // VOWELS - Diphthongs
    /// /aʊ/ - (mouth, how)
    DiphthongAU,
    /// /aɪ/ - (price, my)
    DiphthongAI,
    /// /eɪ/ - (face, say)
    DiphthongEI,
    /// /oʊ/ - (goat, know)
    DiphthongOU,
    /// /ɔɪ/ - (choice, boy)
    DiphthongOI,

    // CONSONANTS - Stops (Plosives)
    /// /p/ - Voiceless bilabial stop (pat)
    VoicelessBilabialStop,
    /// /b/ - Voiced bilabial stop (bat)
    VoicedBilabialStop,
    /// /t/ - Voiceless alveolar stop (tap)
    VoicelessAlveolarStop,
    /// /d/ - Voiced alveolar stop (dad)
    VoicedAlveolarStop,
    /// /k/ - Voiceless velar stop (cat)
    VoicelessVelarStop,
    /// /g/ - Voiced velar stop (gap)
    VoicedVelarStop,
    /// /ʔ/ - Glottal stop (uh-oh)
    GlottalStop,

    // CONSONANTS - Fricatives
    /// /f/ - Voiceless labiodental fricative (fat)
    VoicelessLabiodentalFricative,
    /// /v/ - Voiced labiodental fricative (vat)
    VoicedLabiodentalFricative,
    /// /θ/ - Voiceless dental fricative (thin)
    VoicelessDentalFricative,
    /// /ð/ - Voiced dental fricative (this)
    VoicedDentalFricative,
    /// /s/ - Voiceless alveolar fricative (sat)
    VoicelessAlveolarFricative,
    /// /z/ - Voiced alveolar fricative (zap)
    VoicedAlveolarFricative,
    /// /ʃ/ - Voiceless postalveolar fricative (ship)
    VoicelessPostalveolarFricative,
    /// /ʒ/ - Voiced postalveolar fricative (measure)
    VoicedPostalveolarFricative,
    /// /h/ - Voiceless glottal fricative (hat)
    VoicelessGlottalFricative,

    // CONSONANTS - Affricates
    /// /tʃ/ - Voiceless postalveolar affricate (chip)
    VoicelessPostalveolarAffricate,
    /// /dʒ/ - Voiced postalveolar affricate (jug)
    VoicedPostalveolarAffricate,

    // CONSONANTS - Nasals
    /// /m/ - Bilabial nasal (map)
    BilabialNasal,
    /// /n/ - Alveolar nasal (nap)
    AlveolarNasal,
    /// /ŋ/ - Velar nasal (sing)
    VelarNasal,
    /// /ɲ/ - Palatal nasal (ñ in Spanish señor)
    PalatalNasal,

    // CONSONANTS - Approximants
    /// /l/ - Alveolar lateral approximant (lap)
    AlveolarLateral,
    /// /ɹ/ - Alveolar approximant (rap) - American English R
    AlveolarApproximant,
    /// /r/ - Alveolar trill (Spanish perro) - also used as R variant
    AlveolarTrill,
    /// /w/ - Labial-velar approximant (wag)
    LabialVelarApproximant,
    /// /j/ - Palatal approximant (yes)
    PalatalApproximant,
    /// /ʍ/ - Voiceless labial-velar fricative (which - in some dialects)
    VoicelessLabialVelar,

    // CONSONANTS - Tap/Flap
    /// /ɾ/ - Alveolar tap (better in American English, Filipino r)
    AlveolarTap,

    // MISCALLANEOUS - Not exactly sure what to name it rn
    /// /ː/ - Triangular colon implies that the previous phoneme has a long sound
    TriangularColon,
    /// /:/ - Regular colon implies that the previous phoneme has a long sound
    RegularColon,
}

impl IPA {
    /// Get the IPA character representation
    pub fn as_str(&self) -> &'static str {
        match self {
            // Monophthongs
            IPA::OpenBackUnrounded => "ɑ",
            IPA::NearOpenFront => "æ",
            IPA::OpenMidBack => "ʌ",
            IPA::Schwa => "ə",
            IPA::OpenMidBackRounded => "ɔ",
            IPA::OpenMidFront => "ɛ",
            IPA::OpenMidCentral => "ɜ",
            IPA::RColoredMid => "ɝ",
            IPA::RColoredSchwa => "ɚ",
            IPA::NearCloseFront => "ɪ",
            IPA::CloseFront => "i",
            IPA::NearCloseBack => "ʊ",
            IPA::CloseBack => "u",

            // Diphthongs
            IPA::DiphthongAU => "aʊ",
            IPA::DiphthongAI => "aɪ",
            IPA::DiphthongEI => "eɪ",
            IPA::DiphthongOU => "oʊ",
            IPA::DiphthongOI => "ɔɪ",

            // Stops
            IPA::VoicelessBilabialStop => "p",
            IPA::VoicedBilabialStop => "b",
            IPA::VoicelessAlveolarStop => "t",
            IPA::VoicedAlveolarStop => "d",
            IPA::VoicelessVelarStop => "k",
            IPA::VoicedVelarStop => "g",
            IPA::GlottalStop => "ʔ",

            // Fricatives
            IPA::VoicelessLabiodentalFricative => "f",
            IPA::VoicedLabiodentalFricative => "v",
            IPA::VoicelessDentalFricative => "θ",
            IPA::VoicedDentalFricative => "ð",
            IPA::VoicelessAlveolarFricative => "s",
            IPA::VoicedAlveolarFricative => "z",
            IPA::VoicelessPostalveolarFricative => "ʃ",
            IPA::VoicedPostalveolarFricative => "ʒ",
            IPA::VoicelessGlottalFricative => "h",

            // Affricates
            IPA::VoicelessPostalveolarAffricate => "tʃ",
            IPA::VoicedPostalveolarAffricate => "dʒ",

            // Nasals
            IPA::BilabialNasal => "m",
            IPA::AlveolarNasal => "n",
            IPA::VelarNasal => "ŋ",
            IPA::PalatalNasal => "ɲ",

            // Approximants
            IPA::AlveolarLateral => "l",
            IPA::AlveolarApproximant => "ɹ",
            IPA::AlveolarTrill => "r",
            IPA::LabialVelarApproximant => "w",
            IPA::PalatalApproximant => "j",
            IPA::VoicelessLabialVelar => "ʍ",

            // Tap
            IPA::AlveolarTap => "ɾ",

            // Length
            IPA::TriangularColon => "ː",
            IPA::RegularColon => ":",
        }
    }
}

impl fmt::Display for IPA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
