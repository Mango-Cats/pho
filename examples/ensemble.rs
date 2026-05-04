use pho::{
    algorithms::{Algorithm, JaroWinkler, Levenshtein},
    ensemble::types::{EnsembleAlgorithm, WeightedAlgorithm},
    utils::io::import,
};

fn main() {
    println!("🍜\t| # tutorial: ensemble algorithm");
    // Ensembles
    // ...
    //  Each algorithm has its pros and cons, some have their quirks.
    //
    //  So, an intermediate step would be to combine algorithms to
    //  balance out their cons and maximize the pros we can get from
    //  them!
    //
    //  Ensembling or grouping algorithms together allows us to do
    //  that.
    //
    //  An ensemble of algorithms is basically a list of individual
    //  algorithms and another list of weights. For some input,
    //  the ensemble calls each algorithm in the list and passes in
    //  the input. Then, for each output of the algorithm, the
    //  ensemble scales it with the corresponding weight. The sum
    //  of the scaled values is the result of the ensemble!

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

    // Run the individual components
    // ...
    //  Let's also run the individual components to get an idea on how
    //  each algorithm and their associated weights contributed to the
    //  algorithm.
    let levenshtein_score = levenshtein.similarity(x, y).unwrap();
    let jaro_winkler = jaro_winkler.similarity(x, y).unwrap();

    // Run the ensemble similarity computation
    let score = ensemble.similarity(x, y).unwrap();

    println!("\t| Ensemble Similarity: {score}");
    println!("\t|");
    println!("\t| Levenshtein: {levenshtein_score} (* {w1})");
    println!("\t| Jaro-Winkler: {jaro_winkler} (* {w2})");
}
