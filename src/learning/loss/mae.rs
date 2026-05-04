use crate::dataset::Dataset;
use crate::learning::loss::{types::FitnessEvaluator, util::predict};

/// Mean Absolute Error (MAE) Evaluator.
/// Scales linearly with error size. Best for datasets with noisy labels or outliers.
pub struct MeanAbsoluteError {
    pub precomputed_scores: Vec<Vec<f32>>,
    pub targets: Vec<f32>,
}

impl MeanAbsoluteError {
    pub fn new(data: &Dataset) -> Self {
        Self {
            precomputed_scores: data.base_scores.clone(),
            targets: data.targets.clone(),
        }
    }
}

impl FitnessEvaluator for MeanAbsoluteError {
    fn evaluate(&self, weights: &[f32]) -> f32 {
        let mut sum_absolute_error = 0.0;
        for (base_scores, &target) in self.precomputed_scores.iter().zip(self.targets.iter()) {
            let pred = predict(base_scores, weights);
            sum_absolute_error += (pred - target).abs();
        }

        // Return raw MAE (Lower is better)
        sum_absolute_error / self.targets.len() as f32
    }
}
