#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnsembleConfig {
    /// No limits on weights (can be negative, no sum requirement).
    Linear,
    /// Weights must be >= 0.0.
    Conical,
    /// Weights must sum to 1.0, but can be negative.
    Affine,
    /// Weights must sum to 1.0 and must be >= 0.0 (Probability Distribution).
    Convex,
}
