//! The code below parses feature types for the Aline algorithm
//! from a TOML file.

use std::fs;
use toml::Value;

use super::types::AlineConfig;
use crate::algorithms::aline::{
    cost::Costs,
    features::{
        Back, BackValues, Binary, BinaryValues, ConsonantFeatures, FeatureValues, High, HighValues,
        Manner, MannerValues, PhoneticFeatures, Place, PlaceValues, VowelFeatures,
    },
    salience::Salience,
};

pub fn config_from_toml(file_name: &str) -> Result<AlineConfig, String> {
    if !file_name.ends_with(".toml") {
        return Err("file must be a .toml".to_string());
    }
    let content = fs::read_to_string(file_name).map_err(|e| e.to_string())?;
    let root = content
        .parse::<Value>()
        .map_err(|e| format!("TOML parse error: {e}"))?;
    Ok(AlineConfig {
        costs: parse_costs(&root)?,
        salience: parse_salience(&root)?,
        values: parse_feature_values(&root)?,
        sounds: parse_sounds(&root)?,
    })
}

fn parse_costs(root: &Value) -> Result<Costs, String> {
    let t = section(root, "costs")?;
    Ok(Costs {
        skip: get_i32(t, "skip")?,
        substitute: get_i32(t, "substitute")?,
        expand_compress: get_i32(t, "expand_compress")?,
        vowel_consonant: get_i32(t, "vowel_consonant")?,
    })
}

fn parse_salience(root: &Value) -> Result<Salience, String> {
    let t = section(root, "salience")?;
    Ok(Salience {
        syllabic: get_u32(t, "syllabic")?,
        place: get_u32(t, "place")?,
        manner: get_u32(t, "manner")?,
        voice: get_u32(t, "voice")?,
        nasal: get_u32(t, "nasal")?,
        retroflex: get_u32(t, "retroflex")?,
        lateral: get_u32(t, "lateral")?,
        aspirated: get_u32(t, "aspirated")?,
        long: get_u32(t, "long")?,
        high: get_u32(t, "high")?,
        back: get_u32(t, "back")?,
        round: get_u32(t, "round")?,
    })
}

fn parse_feature_values(root: &Value) -> Result<FeatureValues, String> {
    Ok(FeatureValues {
        place: parse_place_values(root)?,
        manner: parse_manner_values(root)?,
        high: parse_high_values(root)?,
        back: parse_back_values(root)?,
        binary: parse_binary_values(root)?,
    })
}

fn parse_place_values(root: &Value) -> Result<PlaceValues, String> {
    let t = section(root, "place_values")?;
    Ok(PlaceValues {
        bilabial: get_f32(t, "bilabial")?,
        labiodental: get_f32(t, "labiodental")?,
        dental: get_f32(t, "dental")?,
        alveolar: get_f32(t, "alveolar")?,
        retroflex: get_f32(t, "retroflex")?,
        palato_alveolar: get_f32(t, "palato_alveolar")?,
        palatal: get_f32(t, "palatal")?,
        velar: get_f32(t, "velar")?,
        uvular: get_f32(t, "uvular")?,
        pharyngeal: get_f32(t, "pharyngeal")?,
        glottal: get_f32(t, "glottal")?,
        labiovelar: get_f32(t, "labiovelar")?,
        vowel: get_f32(t, "vowel")?,
    })
}

fn parse_manner_values(root: &Value) -> Result<MannerValues, String> {
    let t = section(root, "manner_values")?;
    Ok(MannerValues {
        stop: get_f32(t, "stop")?,
        affricate: get_f32(t, "affricate")?,
        fricative: get_f32(t, "fricative")?,
        trill: get_f32(t, "trill")?,
        tap: get_f32(t, "tap")?,
        approximant: get_f32(t, "approximant")?,
        high_vowel: get_f32(t, "high_vowel")?,
        mid_vowel: get_f32(t, "mid_vowel")?,
        low_vowel: get_f32(t, "low_vowel")?,
        vowel: get_f32(t, "vowel")?,
    })
}

fn parse_high_values(root: &Value) -> Result<HighValues, String> {
    let t = section(root, "height_values")?;
    Ok(HighValues {
        high: get_f32(t, "high")?,
        mid: get_f32(t, "mid")?,
        low: get_f32(t, "low")?,
    })
}

fn parse_back_values(root: &Value) -> Result<BackValues, String> {
    let t = section(root, "backness_values")?;
    Ok(BackValues {
        front: get_f32(t, "front")?,
        central: get_f32(t, "central")?,
        back: get_f32(t, "back")?,
    })
}

fn parse_binary_values(root: &Value) -> Result<BinaryValues, String> {
    let t = section(root, "binary_values")?;
    Ok(BinaryValues {
        plus: get_f32(t, "plus")?,
        minus: get_f32(t, "minus")?,
    })
}

fn parse_sounds(
    root: &Value,
) -> Result<std::collections::HashMap<String, PhoneticFeatures>, String> {
    let sounds_table = root
        .get("sounds")
        .and_then(|v| v.as_table())
        .ok_or("missing [sounds] section")?;

    sounds_table
        .iter()
        .map(|(symbol, entry)| {
            parse_sound(symbol, entry).map(|features| (symbol.clone(), features))
        })
        .collect()
}

fn parse_sound(symbol: &str, entry: &Value) -> Result<PhoneticFeatures, String> {
    let sound_type = entry
        .get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("sound '{symbol}' missing 'type' field"))?;

    match sound_type {
        "consonant" => Ok(PhoneticFeatures::Consonant(parse_consonant(symbol, entry)?)),
        "vowel" => Ok(PhoneticFeatures::Vowel(parse_vowel(symbol, entry)?)),
        other => Err(format!("sound '{symbol}' has unknown type '{other}'")),
    }
}

fn parse_consonant(symbol: &str, t: &Value) -> Result<ConsonantFeatures, String> {
    Ok(ConsonantFeatures {
        aspirated: get_binary(t, "aspirated", symbol)?,
        lateral: get_binary(t, "lateral", symbol)?,
        manner: get_manner(t, "manner", symbol)?,
        nasal: get_binary(t, "nasal", symbol)?,
        place: get_place(t, "place", symbol)?,
        retroflex: get_binary(t, "retroflex", symbol)?,
        syllabic: get_binary(t, "syllabic", symbol)?,
        voice: get_binary(t, "voice", symbol)?,
    })
}

fn parse_vowel(symbol: &str, t: &Value) -> Result<VowelFeatures, String> {
    Ok(VowelFeatures {
        // vowel-only features
        back: get_back(t, "back", symbol)?,
        high: get_high(t, "high", symbol)?,
        round: get_binary(t, "round", symbol)?,
        long: get_binary(t, "long", symbol)?,
        // shared features — vowels need these for sigma
        place: get_place(t, "place", symbol)?,
        manner: get_manner(t, "manner", symbol)?,
        lateral: get_binary(t, "lateral", symbol)?,
        nasal: get_binary(t, "nasal", symbol)?,
        retroflex: get_binary(t, "retroflex", symbol)?,
        syllabic: get_binary(t, "syllabic", symbol)?,
        voice: get_binary(t, "voice", symbol)?,
    })
}

fn get_binary(t: &Value, key: &str, symbol: &str) -> Result<Binary, String> {
    match get_str(t, key, symbol)? {
        "plus" => Ok(Binary::Plus),
        "minus" => Ok(Binary::Minus),
        other => Err(format!(
            "sound '{symbol}': invalid binary value '{other}' for '{key}'"
        )),
    }
}

fn get_place(t: &Value, key: &str, symbol: &str) -> Result<Place, String> {
    match get_str(t, key, symbol)? {
        "bilabial" => Ok(Place::Bilabial),
        "labiodental" => Ok(Place::Labiodental),
        "dental" => Ok(Place::Dental),
        "alveolar" => Ok(Place::Alveolar),
        "retroflex" => Ok(Place::Retroflex),
        "palato_alveolar" => Ok(Place::PalatoAlveolar),
        "palatal" => Ok(Place::Palatal),
        "velar" => Ok(Place::Velar),
        "uvular" => Ok(Place::Uvular),
        "pharyngeal" => Ok(Place::Pharyngeal),
        "glottal" => Ok(Place::Glottal),
        "labiovelar" => Ok(Place::Labiovelar),
        "vowel" => Ok(Place::Vowel),
        other => Err(format!("sound '{symbol}': unknown place '{other}'")),
    }
}

fn get_manner(t: &Value, key: &str, symbol: &str) -> Result<Manner, String> {
    match get_str(t, key, symbol)? {
        "stop" => Ok(Manner::Stop),
        "affricate" => Ok(Manner::Affricate),
        "fricative" => Ok(Manner::Fricative),
        "trill" => Ok(Manner::Trill),
        "tap" => Ok(Manner::Tap),
        "approximant" => Ok(Manner::Approximant),
        "high_vowel" => Ok(Manner::HighVowel),
        "mid_vowel" => Ok(Manner::MidVowel),
        "low_vowel" => Ok(Manner::LowVowel),
        "vowel" => Ok(Manner::Vowel),
        other => Err(format!("sound '{symbol}': unknown manner '{other}'")),
    }
}

fn get_high(t: &Value, key: &str, symbol: &str) -> Result<High, String> {
    match get_str(t, key, symbol)? {
        "high" => Ok(High::High),
        "mid" => Ok(High::Mid),
        "low" => Ok(High::Low),
        other => Err(format!("sound '{symbol}': unknown height '{other}'")),
    }
}

fn get_back(t: &Value, key: &str, symbol: &str) -> Result<Back, String> {
    match get_str(t, key, symbol)? {
        "front" => Ok(Back::Front),
        "central" => Ok(Back::Central),
        "back" => Ok(Back::Back),
        other => Err(format!("sound '{symbol}': unknown backness '{other}'")),
    }
}

fn section<'a>(root: &'a Value, name: &str) -> Result<&'a Value, String> {
    root.get(name)
        .ok_or_else(|| format!("missing [{name}] section"))
}

fn get_str<'a>(t: &'a Value, key: &str, symbol: &str) -> Result<&'a str, String> {
    t.get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("sound '{symbol}' missing or invalid field '{key}'"))
}

fn get_i32(t: &Value, key: &str) -> Result<i32, String> {
    t.get(key)
        .and_then(|v| v.as_integer())
        .map(|v| v as i32)
        .ok_or_else(|| format!("missing or invalid i32 field '{key}'"))
}

fn get_u32(t: &Value, key: &str) -> Result<u32, String> {
    t.get(key)
        .and_then(|v| v.as_integer())
        .map(|v| v as u32)
        .ok_or_else(|| format!("missing or invalid u32 field '{key}'"))
}

fn get_f32(t: &Value, key: &str) -> Result<f32, String> {
    t.get(key)
        .and_then(|v| v.as_float())
        .map(|v| v as f32)
        .ok_or_else(|| format!("missing or invalid f32 field '{key}'"))
}
