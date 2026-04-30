use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Costs {
    pub same: f32,
    pub diff: f32,
}
