//! aline
//!
//! A Rust implementation of the ALINE phonetic similarity algorithm.
//!
//! This ports the core dynamic-programming scoring logic from Kondrak (2002)
//! (and mirrors NLTK's reference implementation in `.dev/references/actual_aline.py`).
//!
//! ## What ALINE computes
//!
//! - `alignment_score(a, b)` computes the *raw* optimal **local** alignment score
//!   between two phonetic segment sequences.
//! - `similarity(a, b)` computes a normalized score in $[0, 1]$:
//!   $$\text{similarity}(a,b) = \frac{\text{alignment\_score}(a,b)}{\max(\text{alignment\_score}(a,a),\,\text{alignment\_score}(b,b))}$$
//!
//! Local alignment means the DP can restart at 0 (Smith–Waterman style), so the
//! best-matching subsequences dominate the score.
//!
//! ## Segments and Unicode
//!
//! IPA strings may contain multi-codepoint graphemes (e.g. letters with
//! combining diacritics). To avoid splitting these incorrectly, inputs are
//! tokenized into Unicode grapheme clusters.

use crate::algorithms::aline::{
    config::AlineConfig,
    features::{Binary, FeatureValues},
    phonemes::PhoneticFeatures,
    salience::Salience,
};
use std::{error::Error, fmt};

pub mod config;
pub mod cost;
mod de;
pub mod features;
pub mod phonemes;
pub mod salience;

/// Error returned when an input string contains a segment that is not present
/// in the configured sound inventory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownSegmentError {
    pub segment: String,
    /// 0-based position in the segmented sequence.
    pub position: usize,
    /// Which input this occurred in (e.g. "x" or "y").
    pub input_name: &'static str,
}

impl fmt::Display for UnknownSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Unknown segment '{}' at position {} in {} (not found in ALINE config sound inventory)",
            self.segment, self.position, self.input_name
        )
    }
}

impl Error for UnknownSegmentError {}

/// Compute normalized phonetic similarity between two IPA strings.
///
/// Returns a score in $[0, 1]$ where 1.0 means identical and 0.0 means
/// maximally dissimilar under the configured costs and feature weights.
pub fn similarity(x: &str, y: &str, config: &AlineConfig) -> Result<f32, UnknownSegmentError> {
    let x_segments = tokenize_and_validate(x, config, "x")?;
    let y_segments = tokenize_and_validate(y, config, "y")?;

    let score = alignment_score(&x_segments, &y_segments, config);

    // Get the possible maximum similarity score to normalize `score`
    let x_self = alignment_score(&x_segments, &x_segments, config);
    let y_self = alignment_score(&y_segments, &y_segments, config);
    let denom = x_self.max(y_self);

    if denom == 0.0 {
        return Ok(0.0);
    }
    Ok(score / denom)
}

/// Split an IPA string into grapheme clusters, and validate each segment exists
/// in the configured inventory.
fn tokenize_and_validate(
    input: &str,
    config: &AlineConfig,
    input_name: &'static str,
) -> Result<Vec<String>, UnknownSegmentError> {
    use unicode_segmentation::UnicodeSegmentation;

    let segments: Vec<String> = UnicodeSegmentation::graphemes(input, true)
        .map(str::to_string)
        .collect();

    for (idx, segment) in segments.iter().enumerate() {
        if !config.sounds.contains_key(segment) {
            return Err(UnknownSegmentError {
                segment: segment.clone(),
                position: idx,
                input_name,
            });
        }
    }

    Ok(segments)
}

/// Raw optimal local alignment score.
///
/// Mirrors NLTK's `_align_score` DP, including expansion/compression edits.
fn alignment_score(x: &[String], y: &[String], config: &AlineConfig) -> f32 {
    let m = x.len();
    let n = y.len();

    // Flattened (m+1) x (n+1) DP matrix. Initialized to 0.0.
    let mut s = vec![0.0f32; (m + 1) * (n + 1)];
    let idx = |i: usize, j: usize| -> usize { i * (n + 1) + j };

    let mut best = 0.0f32;

    for i in 1..=m {
        for j in 1..=n {
            let delete_score = s[idx(i - 1, j)] + indel_score(config);
            let insert_score = s[idx(i, j - 1)] + indel_score(config);
            let substitute_score =
                s[idx(i - 1, j - 1)] + substitution_score(&x[i - 1], &y[j - 1], config);

            let expand_x_score = if i > 1 {
                s[idx(i - 2, j - 1)] + expansion_score(&y[j - 1], &x[i - 2], &x[i - 1], config)
            } else {
                f32::NEG_INFINITY
            };

            let expand_y_score = if j > 1 {
                s[idx(i - 1, j - 2)] + expansion_score(&x[i - 1], &y[j - 2], &y[j - 1], config)
            } else {
                f32::NEG_INFINITY
            };

            let cell = delete_score
                .max(insert_score)
                .max(substitute_score)
                .max(expand_x_score)
                .max(expand_y_score)
                .max(0.0);

            s[idx(i, j)] = cell;
            best = best.max(cell);
        }
    }

    best
}

/// Score for an insertion/deletion (indel). Constant in ALINE.
#[inline]
fn indel_score(config: &AlineConfig) -> f32 {
    config.costs.skip as f32
}

/// Score for substituting one segment for another.
///
/// Mirrors NLTK's `sigma_sub(p, q)`:
/// `C_sub - delta(p, q) - V(p) - V(q)`
#[inline]
fn substitution_score(p: &str, q: &str, config: &AlineConfig) -> f32 {
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
fn expansion_score(p: &str, q1: &str, q2: &str, config: &AlineConfig) -> f32 {
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
