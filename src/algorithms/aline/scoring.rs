use super::Segment;
use super::config::{
    Aline, AlineVariant, Binary, FeatureValues, PhoneticFeatures, Salience, StressOn,
};

/// Score for an insertion/deletion (indel). Constant in ALINE.
#[inline]
pub(crate) fn indel_score(config: &Aline) -> f32 {
    config.costs.skip as f32
}

/// Score for substituting one segment for another.
///
/// Mirrors NLTK's `sigma_sub(p, q)`:
/// `C_sub - delta(p, q) - V(p) - V(q) - S_stress * |stress_p - stress_q|`
///
/// The stress term is only non-zero when `salience.stress > 0` and stress
/// levels are parsed (MangoCats variant). With `stress_on = "vowels"` it only
/// applies when both `p` and `q` are vowels.
#[inline]
pub(crate) fn substitution_score(p: &Segment, q: &Segment, config: &Aline) -> f32 {
    let c_sub = config.costs.substitute as f32;
    let stress_applies = match config.stress_on {
        StressOn::All => true,
        StressOn::Vowels => is_vowel(p, config) && is_vowel(q, config),
    };
    let stress_term = if stress_applies {
        config.salience.stress as f32 * (p.stress - q.stress).abs()
    } else {
        0.0
    };
    c_sub
        - feature_distance(p, q, &config.values, &config.salience, config)
        - vowel_weight(p, config)
        - vowel_weight(q, config)
        - stress_term
}

/// Score for expansion/compression: one segment aligned to two segments.
///
/// Mirrors NLTK's `sigma_exp(p, q1q2)`:
/// `C_exp - delta(p, q1) - delta(p, q2) - V(p) - max(V(q1), V(q2)) - S_stress * |stress_p - max(stress_q1, stress_q2)|`
///
/// With `stress_on = "vowels"` the stress term applies only when `p` is a
/// vowel, and only the vowel(s) among `q1`, `q2` contribute to the stress of
/// the pair; if neither is a vowel the term is 0.
#[inline]
pub(crate) fn expansion_score(p: &Segment, q1: &Segment, q2: &Segment, config: &Aline) -> f32 {
    let c_exp = config.costs.expand_compress as f32;
    let v_p = vowel_weight(p, config);
    let v_q = vowel_weight(q1, config).max(vowel_weight(q2, config));
    let stress_q = match config.stress_on {
        StressOn::All => Some(q1.stress.max(q2.stress)),
        StressOn::Vowels if is_vowel(p, config) => [q1, q2]
            .into_iter()
            .filter(|q| is_vowel(q, config))
            .map(|q| q.stress)
            .reduce(f32::max),
        StressOn::Vowels => None,
    };
    let stress_term = stress_q.map_or(0.0, |sq| {
        config.salience.stress as f32 * (p.stress - sq).abs()
    });
    c_exp
        - feature_distance(p, q1, &config.values, &config.salience, config)
        - feature_distance(p, q2, &config.values, &config.salience, config)
        - v_p
        - v_q
        - stress_term
}

#[inline]
fn is_vowel(segment: &Segment, config: &Aline) -> bool {
    config
        .sounds
        .get(segment.sym.as_str())
        .is_some_and(PhoneticFeatures::is_vowel)
}

/// Vowel/consonant relative weight.
///
/// Mirrors NLTK's `V(p)`: 0 for consonants, `C_vwl` for vowels.
#[inline]
fn vowel_weight(segment: &Segment, config: &Aline) -> f32 {
    let Some(sound) = config.sounds.get(segment.sym.as_str()) else {
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
    p: &Segment,
    q: &Segment,
    values: &FeatureValues,
    salience: &Salience,
    config: &Aline,
) -> f32 {
    let p_sound = &config.sounds[p.sym.as_str()];
    let q_sound = &config.sounds[q.sym.as_str()];
    let extended = matches!(config.variant, AlineVariant::MangoCats);

    if p_sound.is_consonant() || q_sound.is_consonant() {
        // Consonant length (`ː` after a consonant, i.e. gemination) is parsed
        // into `Segment::long` but R_c has no length feature, so it is ignored.
        // TODO: decide whether gemination should be scored for consonants.
        consonant_feature_distance(p_sound, q_sound, values, salience, extended)
    } else {
        vowel_feature_distance(p, p_sound, q, q_sound, values, salience, extended)
    }
}

/// Kondrak R_c: aspirated, lateral, manner, nasal, place, retroflex, syllabic, voice.
/// MangoCats adds: phonation, airstream, secondary.
#[inline]
fn consonant_feature_distance(
    p: &PhoneticFeatures,
    q: &PhoneticFeatures,
    values: &FeatureValues,
    salience: &Salience,
    extended: bool,
) -> f32 {
    let p_common = p.common();
    let q_common = q.common();

    let p_asp = aspirated_or_minus(p);
    let q_asp = aspirated_or_minus(q);

    let mut dist = salience.place as f32
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
        + salience.aspirated as f32 * (values.binary[p_asp] - values.binary[q_asp]).abs();

    if extended {
        dist += salience.phonation as f32
            * (values.phonation[*p_common.phonation()] - values.phonation[*q_common.phonation()])
                .abs()
            + salience.airstream as f32
                * (values.airstream[*p_common.airstream()]
                    - values.airstream[*q_common.airstream()])
                .abs()
            + salience.secondary as f32
                * (values.secondary[*p_common.secondary()]
                    - values.secondary[*q_common.secondary()])
                .abs();
    }

    dist
}

/// Kondrak R_v: back, lateral, long, manner, nasal, place, retroflex, round, syllabic, voice.
/// (`high` is intentionally excluded — it is encoded in `manner` as high/mid/low vowel.)
/// MangoCats adds: phonation, secondary.
///
/// The `long` value of each side comes from the segment's length mark
/// (`ː` / `ˑ`) when present, otherwise from the inventory entry.
#[inline]
fn vowel_feature_distance(
    p_seg: &Segment,
    p: &PhoneticFeatures,
    q_seg: &Segment,
    q: &PhoneticFeatures,
    values: &FeatureValues,
    salience: &Salience,
    extended: bool,
) -> f32 {
    let PhoneticFeatures::Vowel(pv) = p else {
        return 0.0;
    };
    let PhoneticFeatures::Vowel(qv) = q else {
        return 0.0;
    };

    let p_common = &pv.common;
    let q_common = &qv.common;
    let p_long = p_seg.long.unwrap_or(values.binary[pv.long]);
    let q_long = q_seg.long.unwrap_or(values.binary[qv.long]);

    let mut dist = salience.back as f32 * (values.back[pv.back] - values.back[qv.back]).abs()
        + salience.lateral as f32
            * (values.binary[p_common.lateral] - values.binary[q_common.lateral]).abs()
        + salience.long as f32 * (p_long - q_long).abs()
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
            * (values.binary[p_common.voice] - values.binary[q_common.voice]).abs();

    if extended {
        dist += salience.phonation as f32
            * (values.phonation[p_common.phonation] - values.phonation[q_common.phonation]).abs()
            + salience.secondary as f32
                * (values.secondary[p_common.secondary] - values.secondary[q_common.secondary])
                    .abs();
    }

    dist
}

#[inline]
fn aspirated_or_minus(sound: &PhoneticFeatures) -> Binary {
    match sound {
        PhoneticFeatures::Consonant(c) => c.aspirated,
        PhoneticFeatures::Vowel(_) => Binary::Minus,
    }
}
