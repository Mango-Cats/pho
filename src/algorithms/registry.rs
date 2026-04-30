use crate::algorithms::{aline, editex, jaro_winkler, levenshtein};

pub enum AlgorithmConfig {
    AlineConfig(aline::config::AlineConfig),
    EditexConfig(editex::config::EditexConfig),
    JaroWinklerConfig(jaro_winkler::config::JaroWinklerConfig),
    LevenshteinConfig(levenshtein::config::LevenshteinConfig),
}
