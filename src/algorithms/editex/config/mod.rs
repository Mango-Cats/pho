use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod costs;

pub use costs::Costs;

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
