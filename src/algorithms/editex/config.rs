use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::cost::Cost;

#[derive(Debug, Deserialize, Serialize)]
pub struct EditexConfig {
    pub costs: Cost,
    pub group: HashMap<char, Vec<usize>>,
}
