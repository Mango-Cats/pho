use crate::algorithms::aline::{
    cost::Costs,
    features::{FeatureValues, PhoneticFeatures},
    salience::Salience,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct AlineConfig {
    pub costs: Costs,
    pub salience: Salience,
    pub values: FeatureValues,
    pub sounds: HashMap<String, PhoneticFeatures>,
}
