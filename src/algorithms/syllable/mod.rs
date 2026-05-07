//! syllable
//!
//! Syllable-bigram overlap similarity for drug names.
//!
//! Drug names carry strong pharmacological meaning in their syllabic
//! structure: shared suffixes like *-afil*, *-olol*, *-pril*, *-statin*
//! identify drug classes. This algorithm syllabifies each name via English
//! maximal-onset rules, extracts overlapping bigrams of consecutive
//! syllables, then computes a set-overlap metric (Dice, Jaccard, or Overlap
//! coefficient) on the bigram sets.
//!
//! A pair with no syllable bigrams (single-syllable words) falls back to
//! exact-syllable match (1.0 or 0.0).

pub mod config;
mod syllabify;

use std::collections::HashSet;

use crate::{algorithms::Algorithm, error::Result};
use config::{Syllable, SyllableMetric};
use syllabify::syllable_bigrams;

fn set_similarity(
    x: &HashSet<(String, String)>,
    y: &HashSet<(String, String)>,
    metric: &SyllableMetric,
) -> f32 {
    let intersection = x.intersection(y).count() as f32;
    match metric {
        SyllableMetric::Dice => {
            let denom = (x.len() + y.len()) as f32;
            if denom == 0.0 { 1.0 } else { 2.0 * intersection / denom }
        }
        SyllableMetric::Jaccard => {
            let union = (x.len() + y.len()) as f32 - intersection;
            if union == 0.0 { 1.0 } else { intersection / union }
        }
        SyllableMetric::Overlap => {
            let denom = x.len().min(y.len()) as f32;
            if denom == 0.0 { 1.0 } else { intersection / denom }
        }
    }
}

impl Algorithm for Syllable {
    fn similarity(&self, x: &str, y: &str) -> Result<f32> {
        let (xs, ys) = if self.case_insensitive {
            (x.to_lowercase(), y.to_lowercase())
        } else {
            (x.to_string(), y.to_string())
        };

        let x_bigrams: HashSet<(String, String)> = syllable_bigrams(&xs).into_iter().collect();
        let y_bigrams: HashSet<(String, String)> = syllable_bigrams(&ys).into_iter().collect();

        // Single-syllable words produce no bigrams — fall back to full-syllable
        // exact match.
        if x_bigrams.is_empty() && y_bigrams.is_empty() {
            return Ok(if xs == ys { 1.0 } else { 0.0 });
        }

        Ok(set_similarity(&x_bigrams, &y_bigrams, &self.metric).clamp(0.0, 1.0))
    }
}
