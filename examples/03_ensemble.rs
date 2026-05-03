use pho::{
    algorithms::{Algorithm, JaroWinkler, Levenshtein},
    ensemble::types::{EnsembleAlgorithm, WeightedAlgorithm},
    utils::io::import,
};

fn main() {
    println!("🍜\t| # Example 3: ensemble algorithm");

    // Load configs for each algorithm
    let levenshtein = import::<Levenshtein>("tests/config_sample_levenshtein.toml").unwrap();
    let jaro_winkler = import::<JaroWinkler>("tests/config_sample_jaro_winkler.toml").unwrap();

    // Validate configs that define invariants
    jaro_winkler.validate().unwrap();

    // Define the words to compare
    let x = "dixon";
    let y = "dickson";

    // Define the weights
    let w1 = 0.6;
    let w2 = 0.4;

    // Build a weighted ensemble configuration
    // ...
    //  The ensemble algorithm `ensemble` scales the Levenshtein
    //  by 0.6 and Jaro-Winkler by 0.4 and adds them together.
    let ensemble = EnsembleAlgorithm {
        algorithms: vec![
            WeightedAlgorithm::new(levenshtein.clone(), w1),
            WeightedAlgorithm::new(jaro_winkler.clone(), w2),
        ],
        allow_negative_weights: false,
        is_probability_distribution: true,
    };

    // Ensure weights are finite and normalized
    // ...
    //  If the weights assigned do not sum to 1, this will return an
    //  error.
    ensemble.validate().unwrap();

    // Run the ensemble similarity computation
    let score = ensemble.similarity(x, y);

    // Run the individual components
    // ...
    //  Let's also run the individual components to get an idea on how
    //  each algorithm and their associated weights contributed to the
    //  algorithm.
    let levenshtein_score = levenshtein.similarity(x, y).unwrap();
    let jaro_winkler = jaro_winkler.similarity(x, y).unwrap();

    if let Ok(got) = score {
        println!("\t| Ensemble Similarity: {got}");
        println!("\t|");
        println!("\t| Levenshtein: {levenshtein_score} (* {w1})");
        println!("\t| Jaro-Winkler: {jaro_winkler} (* {w2})");
    } else {
        println!("\t| Something went horribly wrong, please raise an issue!")
    }
}
