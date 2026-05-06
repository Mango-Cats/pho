use crate::dataset::ScoreMatrix;
use crate::learning::loss::{types::FitnessEvaluator, util::predict};

/// Mean Squared Error (MSE) Evaluator.
/// Heavily penalizes confident, large errors. Best for general regression.
pub struct MeanSquaredError {
    pub precomputed_scores: Vec<Vec<f32>>,
    pub labels: Vec<f32>,
}

impl MeanSquaredError {
    pub fn new(data: &ScoreMatrix) -> Self {
        let labeled = data
            .base_scores
            .iter()
            .cloned()
            .zip(data.labels.iter().copied())
            .filter_map(|(scores, label)| label.map(|t| (scores, t)))
            .collect::<Vec<_>>();

        Self {
            precomputed_scores: labeled.iter().map(|(scores, _)| scores.clone()).collect(),
            labels: labeled.iter().map(|(_, label)| *label).collect(),
        }
    }
}

impl FitnessEvaluator for MeanSquaredError {
    fn evaluate(&self, weights: &[f32]) -> f32 {
        if self.labels.is_empty() {
            return f32::INFINITY;
        }

        let mut sum_squared_error = 0.0;
        for (base_scores, &label) in self.precomputed_scores.iter().zip(self.labels.iter()) {
            let pred = predict(base_scores, weights);
            let err = pred - label;
            sum_squared_error += err * err;
        }

        sum_squared_error / self.labels.len() as f32
    }
}
