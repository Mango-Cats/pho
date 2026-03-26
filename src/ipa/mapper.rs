use super::symbols::IPA;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static MAPPER: Lazy<HashMap<&'static str, IPA>> = Lazy::new(|| {
    HashMap::from([
        // Diphthongs (check these first - 2 chars)
        ("aʊ", IPA::DiphthongAI),
        ("eɪ", IPA::DiphthongAU),
        ("aɪ", IPA::DiphthongEI),
        ("oʊ", IPA::DiphthongOU),
        ("ɔɪ", IPA::DiphthongOI),
        // Affricates (2 chars)
        ("tʃ", IPA::VoicelessPostalveolarAffricate),
        ("dʒ", IPA::VoicedPostalveolarAffricate),
        // Monophthong vowels
        ("ɑ", IPA::OpenBackUnrounded),
        ("æ", IPA::NearOpenFront),
        ("ʌ", IPA::OpenMidBack),
        ("ɐ", IPA::OpenMidBack),
        ("ə", IPA::Schwa),
        ("ɔ", IPA::OpenMidBackRounded),
        ("ɛ", IPA::OpenMidFront),
        ("ɜ", IPA::OpenMidCentral),
        ("ɝ", IPA::RColoredMid),
        ("ɚ", IPA::RColoredSchwa),
        ("ɪ", IPA::NearCloseFront),
        ("ᵻ", IPA::NearCloseFront),
        ("i", IPA::CloseFront),
        ("ʊ", IPA::NearCloseBack),
        ("u", IPA::CloseBack),
        ("e", IPA::DiphthongEI),
        ("o", IPA::DiphthongOU),
        ("a", IPA::OpenBackUnrounded),
        // Consonants - Stops
        ("p", IPA::VoicelessBilabialStop),
        ("b", IPA::VoicedBilabialStop),
        ("t", IPA::VoicelessAlveolarStop),
        ("d", IPA::VoicedAlveolarStop),
        ("k", IPA::VoicelessVelarStop),
        ("g", IPA::VoicedVelarStop),
        ("ɡ", IPA::VoicedVelarStop),
        ("ʔ", IPA::GlottalStop),
        // Consonants - Fricatives
        ("f", IPA::VoicelessLabiodentalFricative),
        ("v", IPA::VoicedLabiodentalFricative),
        ("θ", IPA::VoicelessDentalFricative),
        ("ð", IPA::VoicedDentalFricative),
        ("s", IPA::VoicelessAlveolarFricative),
        ("z", IPA::VoicedAlveolarFricative),
        ("ʃ", IPA::VoicelessPostalveolarFricative),
        ("ʒ", IPA::VoicedPostalveolarFricative),
        ("h", IPA::VoicelessGlottalFricative),
        // Consonants - Nasals
        ("m", IPA::BilabialNasal),
        ("n", IPA::AlveolarNasal),
        ("ŋ", IPA::VelarNasal),
        ("ɲ", IPA::PalatalNasal),
        // Consonants - Approximants
        ("l", IPA::AlveolarLateral),
        ("ɹ", IPA::AlveolarApproximant),
        ("r", IPA::AlveolarTrill),
        ("w", IPA::LabialVelarApproximant),
        ("j", IPA::PalatalApproximant),
        ("ʍ", IPA::VoicelessLabialVelar),
        // Tap
        ("ɾ", IPA::AlveolarTap),
        // Miscellaneous
        // ("ː", IPA::TriangularColon),
        // (":", IPA::RegularColon),
    ])
});

pub fn str_to_ipa(ipa: &str) -> Vec<IPA> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = ipa.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if i + 1 < chars.len() {
            let two_char: String = chars[i..=i + 1].iter().collect();
            if let Some(symbol) = MAPPER.get(two_char.as_str()) {
                tokens.push(symbol.clone());
                i += 2;
                continue;
            }
        }

        let one_char: String = chars[i..=i].iter().collect();
        if let Some(symbol) = MAPPER.get(one_char.as_str()) {
            tokens.push(symbol.clone());
        }

        i += 1;
    }

    tokens
}
