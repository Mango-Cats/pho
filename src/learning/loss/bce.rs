use crate::learning::{
    dataset::TrainingData,
    loss::{types::FitnessEvaluator, util::predict},
};

/// Binary Cross-Entropy (BCE) Evaluator.
/// The gold standard for classification (e.g., exact matches / duplicates).
pub struct BinaryCrossEntropy {
    pub precomputed_scores: Vec<Vec<f32>>,
    pub targets: Vec<f32>,
}

impl BinaryCrossEntropy {
    pub fn new(data: &TrainingData) -> Self {
        Self {
            precomputed_scores: data.base_scores.clone(),
            targets: data.targets.clone(),
        }
    }
}

impl FitnessEvaluator for BinaryCrossEntropy {
    fn evaluate(&self, weights: &[f32]) -> f32 {
        let mut sum_bce = 0.0;
        let epsilon = 1e-7_f32;

        for (base_scores, &target) in self.precomputed_scores.iter().zip(self.targets.iter()) {
            let pred = predict(base_scores, weights).clamp(epsilon, 1.0 - epsilon);
            sum_bce += target * pred.ln() + (1.0 - target) * (1.0 - pred).ln();
        }

        // Return raw Log Loss (Lower is better)
        -sum_bce / self.targets.len() as f32
    }
}
