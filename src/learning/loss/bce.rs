use crate::dataset::Dataset;
use crate::learning::loss::{types::FitnessEvaluator, util::predict};

/// Binary Cross-Entropy (BCE) Evaluator.
/// The gold standard for classification (e.g., exact matches / duplicates).
pub struct BinaryCrossEntropy {
    pub precomputed_scores: Vec<Vec<f32>>,
    pub labels: Vec<f32>,
}

impl BinaryCrossEntropy {
    pub fn new(data: &Dataset) -> Self {
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

impl FitnessEvaluator for BinaryCrossEntropy {
    fn evaluate(&self, weights: &[f32]) -> f32 {
        if self.labels.is_empty() {
            return f32::INFINITY;
        }

        let mut sum_bce = 0.0;
        let epsilon = 1e-7_f32;

        for (base_scores, &label) in self.precomputed_scores.iter().zip(self.labels.iter()) {
            let pred = predict(base_scores, weights).clamp(epsilon, 1.0 - epsilon);
            sum_bce += label * pred.ln() + (1.0 - label) * (1.0 - pred).ln();
        }

        // Return raw Log Loss (Lower is better)
        -sum_bce / self.labels.len() as f32
    }
}
