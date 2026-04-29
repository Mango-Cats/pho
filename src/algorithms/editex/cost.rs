use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Cost {
    pub same: f32,
    pub diff: f32,
}
