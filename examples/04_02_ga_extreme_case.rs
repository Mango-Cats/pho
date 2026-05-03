use pho::{
    algorithms::{Algorithm, LCS, LCSuf},
    ensemble::types::{EnsembleAlgorithm, WeightedAlgorithm},
    learning::{
        dataset::TrainingData,
        genetic::{GeneticConfig, optimize},
        loss::mse::MeanSquaredError,
    },
};

fn main() {
    println!("🍜\t| # Example 5: Genetic algorithm sanity check (Synthetic Dominance)");
    // Genetic Algorithm (Extreme Case)
    // ...
    //  This example shows the Genetic Algorithm at work in optimizing
    //  the weights of a very bad unoptimized Ensemble.
    //
    //  Here, we also introduce the idea of a training and test set.

    // Build a synthetic dataset
    // ...
    //  We create a dataset that strongly favors Subsequence (LCS) matches
    //  over Suffix (LCSuf) matches. The words share the prefix "banana",
    //  so LCS will return around ~0.4 to 0.5, while LCSuf will return 0.0.
    //  We set the ground truth to match the expected LCS score.
    //
    //  We split this into a training set (for the optimizer) and a test set
    //  (to verify the model generalizes to unseen data without overfitting).

    // The training set is what the Genetic Algorithm will use to adjust the weights.
    let train_set: Vec<(&str, &str, f32)> = vec![
        ("bananamonster", "bananacoolguy", 0.50),
        ("bananaphone", "bananabread", 0.5),
        ("bananaboat", "bananasmoothie", 0.4),
    ];

    // The test set is kept completely hidden from the optimizer. We only use
    // it at the end to evaluate how well our optimized ensemble performs.
    let test_set: Vec<(&str, &str, f32)> = vec![
        ("bananamrsunshine", "bananalucioooo", 0.40),
        ("bananapancakes", "bananasplit", 0.45),
        ("bananaleaf", "bananacake", 0.55),
    ];

    // Build the ensemble with deliberately terrible starting weights
    // ...
    //  We initialize the ensemble with the exact OPPOSITE of what it should be.
    //  The useless algorithm (LCSuf) gets 90% of the weight, while the highly
    //  predictive algorithm (LCS) gets only 10%.
    let mut ensemble = EnsembleAlgorithm {
        algorithms: vec![
            WeightedAlgorithm::new(
                LCS {
                    case_insensitive: true,
                },
                0.1,
            ),
            WeightedAlgorithm::new(
                LCSuf {
                    case_insensitive: true,
                },
                0.9,
            ),
        ],
        allow_negative_weights: false,
        is_probability_distribution: true,
    };

    // Inspect the unoptimised weights
    let w_lcs = ensemble.algorithms[0].weight;
    let w_lcsuf = ensemble.algorithms[1].weight;
    println!("\t|");
    println!("\t| Unoptimized weights (bias towards LCSuf):");
    println!("\t|   LCS   : {w_lcs:.4}");
    println!("\t|   LCSuf : {w_lcsuf:.4}");
    println!("\t|");

    // Configure the genetic algorithm
    // ...
    //  We use the standard configuration. Because the starting weights are
    //  so terrible, the GA should rapidly kill off the LCSuf weight.
    let config = GeneticConfig {
        population_size: 100,
        generations: 50,
        mutation_rate: 0.2,
        mutation_step: 0.1,
        tournament_size: 5,
        elite_count: 2,
    };

    // Precompute dataset
    // ...
    //  To make things fast, we can precompute all values and construct
    //  `TrainingData` from it. Notice we only pass the `train_set` here!
    let training_data = TrainingData::from_ensemble(&ensemble, &train_set).unwrap();

    // Define the evaluator
    // ...
    //  We use MeanSquaredError. The genetic algorithm will heavily penalize
    //  the massive errors caused by the 90% weight on LCSuf.
    let evaluator = MeanSquaredError::new(&training_data);

    println!(
        "\t| Running genetic optimisation ({} generations x {} individuals)",
        config.generations, config.population_size
    );

    // Run the optimizer
    // ...
    //  If the GA works, it should realize LCSuf is useless and flip the weights
    //  so that LCS becomes the dominant algorithm.
    optimize(&mut ensemble, &config, &evaluator).unwrap();

    // Inspect the optimized weights
    let final_lcs = ensemble.algorithms[0].weight;
    let final_lcsuf = ensemble.algorithms[1].weight;

    println!("\t|");
    println!("\t| Optimised weights:");
    println!("\t|   LCS   : {final_lcs:.4}");
    println!("\t|   LCSuf : {final_lcsuf:.4}");

    // Define the deliberately bad baseline to compare against
    let baseline = EnsembleAlgorithm {
        algorithms: vec![
            WeightedAlgorithm::new(
                LCS {
                    case_insensitive: true,
                },
                0.1,
            ),
            WeightedAlgorithm::new(
                LCSuf {
                    case_insensitive: true,
                },
                0.9,
            ),
        ],
        allow_negative_weights: false,
        is_probability_distribution: true,
    };

    // Group the datasets to iterate over both
    let evaluations = [("training set", &train_set), ("unseen test set", &test_set)];

    // Iterate over both datasets to print comparisons
    for (dataset_name, dataset) in evaluations {
        println!("\t|");
        println!("\t| Evaluating on {}:", dataset_name);
        println!(
            "\t| {:─<35} {:─<12} {:─<11} {:─<10} {:─<10}",
            "", "", "", "", ""
        );
        println!(
            "\t| {:<35} {:<12} {:<11} {:<10} {:<10}",
            "pair", "expected", "unoptimized", "optimized", "difference"
        );
        println!(
            "\t| {:─<35} {:─<12} {:─<11} {:─<10} {:─<10}",
            "", "", "", "", ""
        );

        for (a, b, expected) in dataset {
            let label = format!("{a}/{b}");
            let base = baseline.similarity(a, b).unwrap_or(f32::NAN);
            let opt = ensemble.similarity(a, b).unwrap_or(f32::NAN);
            let delta = (opt - expected).abs() - (base - expected).abs();

            println!(
                "\t| {:<35} {:<12.4} {:<11.4} {:<10.4} {:+.4}",
                label, expected, base, opt, delta
            );
        }
    }

    println!("\t|");
    println!("\t| Negative delta = optimized is closer to ground truth on UNSEEN data");

    // Assert our hypothesis! The GA should have inverted the weights.
    assert!(
        final_lcs > final_lcsuf,
        "GA failed to identify the dominant feature! LCS: {}, LCSuf: {}",
        final_lcs,
        final_lcsuf
    );
}
