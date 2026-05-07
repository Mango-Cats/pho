use serde::{Deserialize, Serialize};

/// Configuration for keyboard-proximity edit-distance similarity.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Keyboard {
    /// Cost per insertion (gap in one of the strings).
    pub(crate) insert_cost: f32,
    /// Cost per deletion.
    pub(crate) delete_cost: f32,
    /// Scale factor for substitution cost derived from keyboard distance.
    /// substitution_cost = distance / MAX_KEY_DISTANCE × scale.
    /// A scale of 1.0 keeps substitution cost in [0, 1].
    pub(crate) substitution_scale: f32,
    pub(crate) case_insensitive: bool,
}

impl Keyboard {
    pub fn new(
        insert_cost: f32,
        delete_cost: f32,
        substitution_scale: f32,
        case_insensitive: bool,
    ) -> Self {
        Self { insert_cost, delete_cost, substitution_scale, case_insensitive }
    }
}
