use serde::{Deserialize, Serialize};

/// Cost configuration for Levenshtein distance operations.
///
/// These costs determine the penalty for each edit operation when
/// computing the distance between two strings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cost {
    /// Cost for inserting a character.
    pub insert: f32,
    /// Cost for deleting a character.
    pub delete: f32,
    /// Cost for substituting one character with another.
    pub substitute: f32,
}
