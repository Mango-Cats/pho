use crate::algorithms::{aline, editex, jaro_winkler, levenshtein};

/// This enum contains all the defined algorithms in
/// [crate::algorithms].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    Aline,
    Editex,
    JaroWinkler,
    Levenshtein,
}

/// This enum contains all the configs of those algorithms that
/// require one in [crate::algorithms].
#[derive(Debug)]
pub enum AlgorithmConfig {
    AlineConfig(aline::config::AlineConfig),
    EditexConfig(editex::config::EditexConfig),
    JaroWinklerConfig(jaro_winkler::config::JaroWinklerConfig),
    LevenshteinConfig(levenshtein::config::LevenshteinConfig),
}
