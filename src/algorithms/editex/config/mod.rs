use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod costs;

pub use costs::Costs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Editex {
    pub costs: Costs,
    pub group: HashMap<char, Vec<usize>>,
}
