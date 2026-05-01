use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Costs {
    pub same: f32,
    pub diff: f32,
}
