use pho::{
    algorithms::{AlgorithmTrait, JaroWinkler},
    io,
};

fn main() {
    println!("🍜\t| # Example 2: running an algorithm");

    // Load the config for the Jaro-Winkler algorithm
    let config: JaroWinkler = io::import("tests/config_sample_jaro_winkler.toml").unwrap();
    config.validate().unwrap();

    // Running Algorithms
    // ...
    //  Each algorithm is defined as a struct that holds configs.
    //
    //  By this point, the algorithm assumes that the config passed
    //  through is valid.
    let score = config.similarity("dixon", "dicksonx");
    if let Ok(got) = score {
        println!("Similarity: {got}");
    } else {
        println!("\t| Something went horribly wrong, please raise an issue!")
    }
}
