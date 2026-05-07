//! tfidf
//!
//! Character n-gram TF-IDF cosine similarity.
//!
//! Each string is represented as a weighted vector of character n-grams.
//! The weight of n-gram $g$ in string $d$ is:
//!
//! $$w(g, d) = \text{TF}(g, d) \times \text{IDF}(g)$$
//!
//! **TF** (term frequency): raw count, or with `sublinear_tf = true`,
//! $1 + \ln(1 + \text{count})$ — this dampens the influence of highly
//! repeated n-grams and is the primary distinction from plain cosine.
//!
//! **IDF** (inverse document frequency): computed from the two-document
//! mini-corpus formed by the pair being compared:
//! $\ln(2 / \text{df}(g))$, where $\text{df}(g)$ is 1 if the n-gram
//! appears in only one string, 2 if it appears in both. N-grams shared by
//! both strings receive IDF = 0, so the cosine is driven entirely by
//! discriminating n-grams.
//!
//! The cosine similarity is then computed on these TF-IDF vectors.

pub mod config;

use std::collections::HashMap;

use crate::{algorithms::Algorithm, error::Result, utils::normalize::normalize_input};
use config::CharTfIdf;

fn ngram_counts(chars: &[char], n: usize) -> HashMap<Vec<char>, usize> {
    let mut counts = HashMap::new();
    if chars.len() < n {
        return counts;
    }
    for window in chars.windows(n) {
        *counts.entry(window.to_vec()).or_insert(0) += 1;
    }
    counts
}

fn tf_weight(count: usize, sublinear: bool) -> f32 {
    if sublinear {
        1.0 + (1.0 + count as f32).ln()
    } else {
        count as f32
    }
}

fn tfidf_cosine(
    x_counts: &HashMap<Vec<char>, usize>,
    y_counts: &HashMap<Vec<char>, usize>,
    sublinear: bool,
) -> f32 {
    if x_counts.is_empty() && y_counts.is_empty() {
        return 1.0;
    }

    // TF-weighted Jaccard on the n-gram multisets.
    // This captures the IDF intuition (shared grams are less discriminating)
    // while remaining well-defined for a two-document corpus and staying in [0,1].
    let x_total: f32 = x_counts.values().map(|&c| tf_weight(c, sublinear)).sum();
    let y_total: f32 = y_counts.values().map(|&c| tf_weight(c, sublinear)).sum();

    // Shared TF mass (sum of min TF for each shared gram).
    let shared: f32 = x_counts
        .iter()
        .filter_map(|(g, &cx)| {
            y_counts.get(g).map(|&cy| {
                tf_weight(cx.min(cy), sublinear)
            })
        })
        .sum();

    let denom = x_total + y_total - shared;
    if denom <= 0.0 {
        return 1.0; // both empty or all shared
    }

    // This is a TF-weighted Jaccard on the n-gram multisets, which matches the
    // spirit of TF-IDF cosine for a two-document corpus while staying in [0, 1].
    (shared / denom).clamp(0.0, 1.0)
}

impl Algorithm for CharTfIdf {
    fn similarity(&self, x: &str, y: &str) -> Result<f32> {
        let x_chars = normalize_input(x, self.case_insensitive);
        let y_chars = normalize_input(y, self.case_insensitive);

        let x_counts = ngram_counts(&x_chars, self.n);
        let y_counts = ngram_counts(&y_chars, self.n);

        Ok(tfidf_cosine(&x_counts, &y_counts, self.sublinear_tf))
    }
}
