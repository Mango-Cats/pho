//! aline::config::types
//!
//! This file contains the `AlineConfig` struct which stores similarity
//! values and phonetic features that is used by the Aline algorithm.
use crate::algorithms::aline::{
    cost::Costs,
    features::{FeatureValues, PhoneticFeatures},
    salience::Salience,
};

use std::collections::HashMap;

/// This struct stores the similarity values and phonetic features
/// that is used by the Aline algorithm.
pub struct AlineConfig {
    pub costs: Costs,
    pub salience: Salience,
    pub values: FeatureValues,
    pub sounds: HashMap<String, PhoneticFeatures>,
}
