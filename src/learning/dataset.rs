// src/learning/dataset.rs

use crate::algorithms::Algorithm;
use crate::ensemble::types::EnsembleAlgorithm;
use crate::error::Result;

/// A container for precomputed base scores and their target labels.
/// This drastically speeds up optimization by preventing redundant string comparisons.
#[derive(Debug, Clone)]
pub struct TrainingData {
    pub base_scores: Vec<Vec<f32>>,
    pub targets: Vec<f32>,
}

impl TrainingData {
    // Private helper doing the actual heavy lifting
    fn build<'a, S1, S2, I>(algorithms: I, labeled_data: &[(S1, S2, f32)]) -> Result<Self>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        I: Iterator<Item = &'a dyn Algorithm> + Clone,
    {
        let mut base_scores = Vec::with_capacity(labeled_data.len());
        let mut targets = Vec::with_capacity(labeled_data.len());

        for (a, b, target) in labeled_data {
            let scores = algorithms
                .clone()
                .map(|algo| algo.similarity(a.as_ref(), b.as_ref()))
                .collect::<Result<Vec<f32>>>()?;
            base_scores.push(scores);
            targets.push(*target);
        }

        Ok(Self {
            base_scores,
            targets,
        })
    }

    pub fn from_slice<S1, S2>(
        algorithms: &[&dyn Algorithm],
        labeled_data: &[(S1, S2, f32)],
    ) -> Result<Self>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        Self::build(algorithms.iter().copied(), labeled_data)
    }

    pub fn from_ensemble<S1, S2>(
        ensemble: &EnsembleAlgorithm,
        labeled_data: &[(S1, S2, f32)],
    ) -> Result<Self>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        Self::build(
            ensemble.algorithms.iter().map(|wa| wa.algorithm.as_ref()),
            labeled_data,
        )
    }
}
