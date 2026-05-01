use pho::{
    algorithms::{AlgorithmTrait, JaroWinkler},
    config_io,
};

fn main() {
    println!("# Example 2: running an algorithm");

    // Boilerplate
    let config: JaroWinkler = config_io::import("tests/config_sample_jaro_winkler.toml").unwrap();
    config.validate().unwrap();

    // # Running Algorithms
    //
    // ## Algorithm Structs
    // Each algorithm is defined as a struct that holds configs
    let score = config.similarity("dixon", "dicksonx");
    if let Ok(got) = score {
        println!("Similarity: {got}");
    } else {
        println!("Uh oh!")
    }
}
