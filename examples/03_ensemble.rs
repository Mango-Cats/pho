use pho::{
    algorithms::{
        JaroWinkler, Levenshtein,
        ensemble::{EnsembleAlgorithm, WeightedAlgorithm, similarity},
    },
    config_io,
};

fn main() {
    println!("# Example 3: ensemble algorithm");

    // # Load configs for each algorithm
    let levenshtein =
        config_io::import::<Levenshtein>("tests/config_sample_levenshtein.toml").unwrap();
    let jaro_winkler =
        config_io::import::<JaroWinkler>("tests/config_sample_jaro_winkler.toml").unwrap();

    // # Validate configs that define invariants
    jaro_winkler.validate().unwrap();

    // # Build a weighted ensemble configuration
    let ensemble = EnsembleAlgorithm {
        algorithms: vec![
            WeightedAlgorithm::new(levenshtein, 0.6),
            WeightedAlgorithm::new(jaro_winkler, 0.4),
        ],
    };

    // # Ensure weights are finite and normalized
    ensemble.validate().unwrap();

    // # Run the ensemble similarity computation
    let score = similarity("dixon", "dicksonx", &ensemble);
    if let Ok(got) = score {
        println!("Similarity: {got}");
    } else {
        println!("Uh oh!")
    }
}
