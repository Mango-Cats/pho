use crate::dataset::Dataset;
use crate::learning::loss::{types::FitnessEvaluator, util::predict};

/// Mean Squared Error (MSE) Evaluator.
/// Heavily penalizes confident, large errors. Best for general regression.
pub struct MeanSquaredError {
    pub precomputed_scores: Vec<Vec<f32>>,
    pub targets: Vec<f32>,
}

impl MeanSquaredError {
    pub fn new(data: &Dataset) -> Self {
        Self {
            precomputed_scores: data.base_scores.clone(),
            targets: data.targets.clone(),
        }
    }
}

impl FitnessEvaluator for MeanSquaredError {
    fn evaluate(&self, weights: &[f32]) -> f32 {
        let mut sum_squared_error = 0.0;
        for (base_scores, &target) in self.precomputed_scores.iter().zip(self.targets.iter()) {
            let pred = predict(base_scores, weights);
            let err = pred - target;
            sum_squared_error += err * err;
        }

        sum_squared_error / self.targets.len() as f32
    }
}
