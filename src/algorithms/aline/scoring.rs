use super::config::{AlineConfig, Binary, FeatureValues, PhoneticFeatures, Salience};

/// Score for an insertion/deletion (indel). Constant in ALINE.
#[inline]
pub(crate) fn indel_score(config: &AlineConfig) -> f32 {
    config.costs.skip as f32
}

/// Score for substituting one segment for another.
///
/// Mirrors NLTK's `sigma_sub(p, q)`:
/// `C_sub - delta(p, q) - V(p) - V(q)`
#[inline]
pub(crate) fn substitution_score(p: &str, q: &str, config: &AlineConfig) -> f32 {
    let c_sub = config.costs.substitute as f32;
    c_sub
        - feature_distance(p, q, &config.values, &config.salience, config)
        - vowel_weight(p, config)
        - vowel_weight(q, config)
}

/// Score for expansion/compression: one segment aligned to two segments.
///
/// Mirrors NLTK's `sigma_exp(p, q1q2)`:
/// `C_exp - delta(p, q1) - delta(p, q2) - V(p) - max(V(q1), V(q2))`.
#[inline]
pub(crate) fn expansion_score(p: &str, q1: &str, q2: &str, config: &AlineConfig) -> f32 {
    let c_exp = config.costs.expand_compress as f32;
    let v_p = vowel_weight(p, config);
    let v_q = vowel_weight(q1, config).max(vowel_weight(q2, config));
    c_exp
        - feature_distance(p, q1, &config.values, &config.salience, config)
        - feature_distance(p, q2, &config.values, &config.salience, config)
        - v_p
        - v_q
}

/// Vowel/consonant relative weight.
///
/// Mirrors NLTK's `V(p)`: 0 for consonants, `C_vwl` for vowels.
#[inline]
fn vowel_weight(segment: &str, config: &AlineConfig) -> f32 {
    let Some(sound) = config.sounds.get(segment) else {
        return 0.0;
    };

    if sound.is_consonant() {
        0.0
    } else {
        config.costs.vowel_consonant as f32
    }
}

/// Salience-weighted feature distance (`delta(p, q)` in Kondrak/NLTK).
///
/// The relevant feature set depends on the sound types:
/// - If either segment is a consonant, compare consonant-relevant features.
/// - Otherwise (both vowels), compare vowel-relevant features.
fn feature_distance(
    p: &str,
    q: &str,
    values: &FeatureValues,
    salience: &Salience,
    config: &AlineConfig,
) -> f32 {
    let p_sound = &config.sounds[p];
    let q_sound = &config.sounds[q];

    if p_sound.is_consonant() || q_sound.is_consonant() {
        consonant_feature_distance(p_sound, q_sound, values, salience)
    } else {
        vowel_feature_distance(p_sound, q_sound, values, salience)
    }
}

#[inline]
fn consonant_feature_distance(
    p: &PhoneticFeatures,
    q: &PhoneticFeatures,
    values: &FeatureValues,
    salience: &Salience,
) -> f32 {
    let p_common = p.common();
    let q_common = q.common();

    // In NLTK's feature matrix, vowels still have `aspirated = minus` so that
    // consonant-vowel comparisons can treat aspirated as defined.
    let p_asp = aspirated_or_minus(p);
    let q_asp = aspirated_or_minus(q);

    salience.place as f32
        * (values.place[*p_common.place()] - values.place[*q_common.place()]).abs()
        + salience.manner as f32
            * (values.manner[*p_common.manner()] - values.manner[*q_common.manner()]).abs()
        + salience.syllabic as f32
            * (values.binary[*p_common.syllabic()] - values.binary[*q_common.syllabic()]).abs()
        + salience.voice as f32
            * (values.binary[*p_common.voice()] - values.binary[*q_common.voice()]).abs()
        + salience.nasal as f32
            * (values.binary[*p_common.nasal()] - values.binary[*q_common.nasal()]).abs()
        + salience.retroflex as f32
            * (values.binary[*p_common.retroflex()] - values.binary[*q_common.retroflex()]).abs()
        + salience.lateral as f32
            * (values.binary[*p_common.lateral()] - values.binary[*q_common.lateral()]).abs()
        + salience.aspirated as f32 * (values.binary[p_asp] - values.binary[q_asp]).abs()
}

#[inline]
fn vowel_feature_distance(
    p: &PhoneticFeatures,
    q: &PhoneticFeatures,
    values: &FeatureValues,
    salience: &Salience,
) -> f32 {
    let PhoneticFeatures::Vowel(pv) = p else {
        return 0.0;
    };
    let PhoneticFeatures::Vowel(qv) = q else {
        return 0.0;
    };
    let p_common = &pv.common;
    let q_common = &qv.common;

    // Mirrors NLTK's R_v (note: excludes the redundant `high` feature).
    salience.back as f32 * (values.back[pv.back] - values.back[qv.back]).abs()
        + salience.lateral as f32
            * (values.binary[p_common.lateral] - values.binary[q_common.lateral]).abs()
        + salience.long as f32 * (values.binary[pv.long] - values.binary[qv.long]).abs()
        + salience.manner as f32
            * (values.manner[p_common.manner] - values.manner[q_common.manner]).abs()
        + salience.nasal as f32
            * (values.binary[p_common.nasal] - values.binary[q_common.nasal]).abs()
        + salience.place as f32
            * (values.place[p_common.place] - values.place[q_common.place]).abs()
        + salience.retroflex as f32
            * (values.binary[p_common.retroflex] - values.binary[q_common.retroflex]).abs()
        + salience.round as f32 * (values.binary[pv.round] - values.binary[qv.round]).abs()
        + salience.syllabic as f32
            * (values.binary[p_common.syllabic] - values.binary[q_common.syllabic]).abs()
        + salience.voice as f32
            * (values.binary[p_common.voice] - values.binary[q_common.voice]).abs()
}

#[inline]
fn aspirated_or_minus(sound: &PhoneticFeatures) -> Binary {
    match sound {
        PhoneticFeatures::Consonant(c) => c.aspirated,
        PhoneticFeatures::Vowel(_) => Binary::Minus,
    }
}
