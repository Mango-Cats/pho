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
};

fn normalize_input(input: &str, case_insensitive: bool) -> Vec<char> {
    let normalized = if case_insensitive {
        input.to_lowercase()
    } else {
        input.to_string()
    };

    normalized.chars().collect()
}

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

fn intersection_size(left: &HashSet<Vec<char>>, right: &HashSet<Vec<char>>) -> usize {
    left.intersection(right).count()
}

fn dice_similarity(left: &HashSet<Vec<char>>, right: &HashSet<Vec<char>>) -> f32 {
    let denominator = (left.len() + right.len()) as f32;
    if denominator == 0.0 {
        return 1.0;
    }

    (2.0 * intersection_size(left, right) as f32) / denominator
}

fn jaccard_similarity(left: &HashSet<Vec<char>>, right: &HashSet<Vec<char>>) -> f32 {
    let intersection = intersection_size(left, right);
    let union = left.len() + right.len() - intersection;

    if union == 0 {
        return 1.0;
    }

    intersection as f32 / union as f32
}

fn overlap_similarity(left: &HashSet<Vec<char>>, right: &HashSet<Vec<char>>) -> f32 {
    let intersection = intersection_size(left, right);
    let denominator = left.len().min(right.len()) as f32;

    if denominator == 0.0 {
        return 1.0;
    }

    intersection as f32 / denominator
}

fn tversky_similarity(
    left: &HashSet<Vec<char>>,
    right: &HashSet<Vec<char>>,
    alpha: f32,
    beta: f32,
) -> f32 {
    let intersection = intersection_size(left, right) as f32;
    let left_only = left.difference(right).count() as f32;
    let right_only = right.difference(left).count() as f32;
    let denominator = intersection + (alpha * left_only) + (beta * right_only);

    if denominator == 0.0 {
        return 1.0;
    }

    intersection / denominator
}

fn cosine_similarity(left: &HashMap<Vec<char>, usize>, right: &HashMap<Vec<char>, usize>) -> f32 {
    if left.is_empty() && right.is_empty() {
        return 1.0;
    }

    let dot_product = left.iter().fold(0.0_f32, |acc, (gram, left_count)| {
        acc + right
            .get(gram)
            .map(|right_count| (*left_count as f32) * (*right_count as f32))
            .unwrap_or(0.0)
    });

    let left_norm = left
        .values()
        .fold(0.0_f32, |acc, count| acc + (*count as f32).powi(2))
        .sqrt();
    let right_norm = right
        .values()
        .fold(0.0_f32, |acc, count| acc + (*count as f32).powi(2))
        .sqrt();

    let denominator = left_norm * right_norm;
    if denominator == 0.0 {
        return 0.0;
    }

    (dot_product / denominator).clamp(0.0, 1.0)
}

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
                dice_similarity(&x_grams, &y_grams)
            }
            NGramMetric::Jaccard => {
                let x_grams = unique_ngrams(&x_padded, self.n);
                let y_grams = unique_ngrams(&y_padded, self.n);
                jaccard_similarity(&x_grams, &y_grams)
            }
            NGramMetric::Overlap => {
                let x_grams = unique_ngrams(&x_padded, self.n);
                let y_grams = unique_ngrams(&y_padded, self.n);
                overlap_similarity(&x_grams, &y_grams)
            }
            NGramMetric::Tversky { alpha, beta } => {
                let x_grams = unique_ngrams(&x_padded, self.n);
                let y_grams = unique_ngrams(&y_padded, self.n);
                tversky_similarity(&x_grams, &y_grams, alpha, beta)
            }
            NGramMetric::Cosine => {
                let x_grams = counted_ngrams(&x_padded, self.n);
                let y_grams = counted_ngrams(&y_padded, self.n);
                cosine_similarity(&x_grams, &y_grams)
            }
        };

        Ok(score.clamp(0.0, 1.0))
    }
}
