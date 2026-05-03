use std::collections::HashMap;

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Editex {
    pub(crate) costs: Costs,
    pub(crate) group: HashMap<char, Vec<usize>>,
}

impl Editex {
    pub fn new(costs: Costs, group: HashMap<char, Vec<usize>>) -> Self {
        Self { costs, group }
    }
}
