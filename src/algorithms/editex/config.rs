use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use super::cost::Cost;

#[derive(Debug, Deserialize, Serialize)]
pub struct EditexConfig {
    pub costs: Cost,
    pub group: HashSet<Vec<String>>,
}
