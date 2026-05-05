/// Helper to calculate the ensemble prediction for a single data point
/// given the precomputed base algorithm scores and the current GA weights.
#[inline]
pub(super) fn predict(base_scores: &[f32], weights: &[f32]) -> f32 {
    let aligned_scores = if base_scores.len() == weights.len() + 1 {
        &base_scores[1..]
    } else {
        base_scores
    };

    aligned_scores
        .iter()
        .zip(weights.iter())
        .map(|(score, weight)| score * weight)
        .sum::<f32>()
        .clamp(0.0, 1.0)
}
