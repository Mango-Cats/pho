use crate::algorithms::aline::{
    cost::Costs,
    features::{FeatureValues, PhoneticFeatures},
    salience::Salience,
};

use std::collections::HashMap;

pub struct AlineConfig {
    pub costs: Costs,
    pub salience: Salience,
    pub values: FeatureValues,
    pub sounds: HashMap<String, PhoneticFeatures>,
}
