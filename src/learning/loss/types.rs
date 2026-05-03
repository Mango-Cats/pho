/// A trait to evaluate how well a given set of weights performs.
///
/// Implementors must be `Sync` so fitness evaluation can be parallelized
/// across the population with Rayon.
pub trait FitnessEvaluator: Sync {
    fn evaluate(&self, weights: &[f32]) -> f32;
}
