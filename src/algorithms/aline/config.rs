//! aline::config
//!
//! This file contains the struct that holds the configuration
//! (togglable) values of the Aline algorithm. This allows for the
//! implementation to be flexible and tinkerable.
//!
//! This structure contains the cost variables from [`Costs`], the
//! salience values from [`Salience`], the phonetic feature matrix
//! from [`FeatureValues`], and finally the phonemes inventory through
//! a [`HashMap<String, PhoneticFeatures>`] (this is a mapping of a
//! symbol to its phonetic features).
//!
//! ## Reading from a Config
//!
//! Since the purpose of this section is to allow for easy tinkering.
//! One may implement a set of pre-defined set of values the Aline
//! algorithm can use as a TOML file.
//!
//! This process is desciribed in `algorithms/aline/de.rs` and
//! `algorithms/parser/mod.rs`.
use crate::algorithms::aline::{
    cost::Costs, features::FeatureValues, phonemes::PhoneticFeatures, salience::Salience,
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
