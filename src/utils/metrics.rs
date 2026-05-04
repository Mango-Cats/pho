use std::collections::{HashMap, HashSet};

pub fn intersection_size<T: std::hash::Hash + Eq>(left: &HashSet<T>, right: &HashSet<T>) -> usize {
    left.intersection(right).count()
}

pub fn dice_similarity<T: std::hash::Hash + Eq>(left: &HashSet<T>, right: &HashSet<T>) -> f32 {
    let denominator = (left.len() + right.len()) as f32;
    if denominator == 0.0 {
        return 1.0;
    }

    (2.0 * intersection_size(left, right) as f32) / denominator
}

pub fn jaccard_similarity<T: std::hash::Hash + Eq>(left: &HashSet<T>, right: &HashSet<T>) -> f32 {
    let intersection = intersection_size(left, right);
    let union = left.len() + right.len() - intersection;

    if union == 0 {
        return 1.0;
    }

    intersection as f32 / union as f32
}

pub fn overlap_similarity<T: std::hash::Hash + Eq>(left: &HashSet<T>, right: &HashSet<T>) -> f32 {
    let intersection = intersection_size(left, right);
    let denominator = left.len().min(right.len()) as f32;

    if denominator == 0.0 {
        return 1.0;
    }

    intersection as f32 / denominator
}

pub fn tversky_similarity<T: std::hash::Hash + Eq>(
    left: &HashSet<T>,
    right: &HashSet<T>,
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

pub fn cosine_similarity<
    T: std::hash::Hash + Eq + std::cmp::Eq + std::cmp::PartialEq + std::fmt::Debug,
>(
    left: &HashMap<T, usize>,
    right: &HashMap<T, usize>,
) -> f32 {
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
