use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Costs {
    pub(crate) same: f32,
    pub(crate) diff: f32,
}

impl Costs {
    pub fn new(same: f32, diff: f32) -> Self {
        Self { same, diff }
    }
}
