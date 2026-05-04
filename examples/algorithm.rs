use pho::{
    algorithms::{Algorithm, JaroWinkler},
    utils::io::import,
};

fn main() {
    println!("🍜\t| # tutorial: running an algorithm");
    // Algorithms
    // ...
    //  This example shows how to run algorithms.

    // Load the config for the Jaro-Winkler algorithm
    let my_algorithm: JaroWinkler = import("tests/config_sample_jaro_winkler.toml").unwrap();
    my_algorithm.validate().unwrap();

    // Running Algorithms
    // ...
    //  Each algorithm is defined as a struct that holds configs.
    let score = my_algorithm.similarity("dixon", "dicksonx").unwrap();
    println!("\t| Similarity: {score}");
}
