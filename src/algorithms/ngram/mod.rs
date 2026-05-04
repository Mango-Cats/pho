//! ngram
//!
//! N-gram similarity with configurable left/right space padding.
//!
//! The input string is padded with `before_padding` spaces before and
//! `after_padding` spaces after extraction. Similarity can then be measured
//! with Dice, Jaccard, Overlap, Tversky, or cosine similarity.

pub mod config;

use std::collections::{HashMap, HashSet};

use crate::{
    algorithms::{
        Algorithm,
        ngram::config::{NGram, NGramMetric},
    },
    error::Result,
    utils::{metrics, normalize::normalize_input},
};

fn padded_chars(chars: &[char], before_padding: usize, after_padding: usize) -> Vec<char> {
    let mut padded = Vec::with_capacity(before_padding + chars.len() + after_padding);
    padded.extend(std::iter::repeat(' ').take(before_padding));
    padded.extend(chars.iter().copied());
    padded.extend(std::iter::repeat(' ').take(after_padding));
    padded
}

fn ngrams(chars: &[char], n: usize) -> Vec<Vec<char>> {
    if n == 0 || chars.len() < n {
        return Vec::new();
    }

    chars.windows(n).map(|window| window.to_vec()).collect()
}

fn unique_ngrams(chars: &[char], n: usize) -> HashSet<Vec<char>> {
    ngrams(chars, n).into_iter().collect()
}

fn counted_ngrams(chars: &[char], n: usize) -> HashMap<Vec<char>, usize> {
    let mut counts = HashMap::new();

    for gram in ngrams(chars, n) {
        *counts.entry(gram).or_insert(0) += 1;
    }

    counts
}

// Metric implementations are shared in `crate::utils::metrics`.
impl Algorithm for NGram {
    fn similarity(&self, x: &str, y: &str) -> Result<f32> {
        let x_chars = normalize_input(x, self.case_insensitive);
        let y_chars = normalize_input(y, self.case_insensitive);

        let x_padded = padded_chars(&x_chars, self.before_padding, self.after_padding);
        let y_padded = padded_chars(&y_chars, self.before_padding, self.after_padding);

        let score = match self.metric {
            NGramMetric::Dice => {
                let x_grams = unique_ngrams(&x_padded, self.n);
                let y_grams = unique_ngrams(&y_padded, self.n);
                metrics::dice_similarity(&x_grams, &y_grams)
            }
            NGramMetric::Jaccard => {
                let x_grams = unique_ngrams(&x_padded, self.n);
                let y_grams = unique_ngrams(&y_padded, self.n);
                metrics::jaccard_similarity(&x_grams, &y_grams)
            }
            NGramMetric::Overlap => {
                let x_grams = unique_ngrams(&x_padded, self.n);
                let y_grams = unique_ngrams(&y_padded, self.n);
                metrics::overlap_similarity(&x_grams, &y_grams)
            }
            NGramMetric::Tversky { alpha, beta } => {
                let x_grams = unique_ngrams(&x_padded, self.n);
                let y_grams = unique_ngrams(&y_padded, self.n);
                metrics::tversky_similarity(&x_grams, &y_grams, alpha, beta)
            }
            NGramMetric::Cosine => {
                let x_grams = counted_ngrams(&x_padded, self.n);
                let y_grams = counted_ngrams(&y_padded, self.n);
                metrics::cosine_similarity(&x_grams, &y_grams)
            }
        };

        Ok(score.clamp(0.0, 1.0))
    }
}
