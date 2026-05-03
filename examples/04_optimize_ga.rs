use pho::{
    algorithms::{Algorithm, JaroWinkler, Levenshtein},
    ensemble::types::{EnsembleAlgorithm, WeightedAlgorithm},
    learning::{
        dataset::TrainingData,
        genetic::{GeneticConfig, optimize},
        loss::{bce::BinaryCrossEntropy, mae::MeanAbsoluteError, mse::MeanSquaredError},
    },
    utils::io::import,
};

fn main() {
    println!("🍜\t| # Example 4: genetic weight optimization");

    // Load and validate configs
    // ...
    //  Just like in the ensemble example, we start by importing and
    //  validating the algorithm configs we want to combine.
    let levenshtein = import::<Levenshtein>("tests/config_sample_levenshtein.toml").unwrap();
    let jaro_winkler = import::<JaroWinkler>("tests/config_sample_jaro_winkler.toml").unwrap();
    jaro_winkler.validate().unwrap();

    // Build a labelled dataset
    // ...
    //  The genetic algorithm needs a way to judge how good a set of
    //  weights is. We do that by defining a small ground-truth dataset
    //  of (word_a, word_b, expected_similarity) triples.
    //
    //  The evaluator will score a candidate weight vector by computing
    //  the ensemble similarity for each pair and measuring how close
    //  it lands to the expected value.
    let ground_truth: Vec<(&str, &str, f32)> = vec![
        ("dixon", "dicksonx", 0.81),
        ("martha", "marhta", 0.96),
        ("jellyfish", "smellyfish", 0.73),
        ("cat", "car", 0.83),
        ("saturday", "sunday", 0.62),
        ("bupropion", "buspirone", 0.76),
    ];

    // Build the ensemble with equal starting weights
    // ...
    //  The genetic algorithm will overwrite these weights, so their
    //  initial values don't matter as long as the ensemble is valid.
    let mut ensemble: EnsembleAlgorithm = EnsembleAlgorithm::new_uniform_probability(vec![
        Box::new(levenshtein.clone()),
        Box::new(jaro_winkler.clone()),
    ])
    .unwrap();

    // Inspect the unoptimised weights
    let w_lev = ensemble.algorithms[0].weight;
    let w_jaro = ensemble.algorithms[1].weight;
    println!("\t|");
    println!("\t| Unoptimized weights:");
    println!("\t|   Levenshtein  : {w_lev:.4}");
    println!("\t|   Jaro-Winkler : {w_jaro:.4}");
    println!("\t|");

    // Configure the genetic algorithm
    // ...
    //  `GeneticConfig` exposes all the knobs you'd expect:
    //
    //    population_size  - individuals evaluated per generation
    //    generations      - how many rounds of evolution to run
    //    mutation_rate    - probability of a gene being nudged
    //    mutation_step    - maximum magnitude of a single nudge
    //    tournament_size  - candidates sampled per parent selection
    //    elite_count      - top individuals carried over unchanged
    //
    //  The defaults are a reasonable starting point for most tasks.
    let config = GeneticConfig {
        population_size: 200,
        generations: 100,
        mutation_rate: 0.1,
        mutation_step: 0.05,
        tournament_size: 5,
        elite_count: 3,
    };

    // Precompute dataset
    // ...
    //  To make things fast, we can precompute all values and construct
    //  `TrainingData` from it.
    let training_data = TrainingData::from_ensemble(&ensemble, &ground_truth).unwrap();

    // Define the evaluator
    // ...
    //  Since these are **loss** functions, the genetic algorithm's
    //  goal is to minimize this.
    //
    //  This example uses `MeanSquaredError` but you can change the
    //  loss function.
    let evaluator = MeanAbsoluteError::new(&training_data);
    optimize(&mut ensemble, &config, &evaluator).unwrap();

    println!(
        "\t| Running genetic optimisation ({} generations x {} individuals)",
        config.generations, config.population_size
    );

    // Run the optimizer
    // ...
    //  `optimize` mutates `ensemble` in-place,
    //  replacing each algorithm's weight with the best value found.
    //  It calls `ensemble.validate()` internally before returning, so
    //  a successful result guarantees a valid, normalised ensemble.
    optimize(&mut ensemble, &config, &evaluator).unwrap();

    // Inspect the optimised weights
    let w_lev = ensemble.algorithms[0].weight;
    let w_jaro = ensemble.algorithms[1].weight;

    println!("\t|");
    println!("\t| Optimised weights:");
    println!("\t|   Levenshtein  : {w_lev:.4}");
    println!("\t|   Jaro-Winkler : {w_jaro:.4}");

    // Compare optimised vs. equal-weight ensemble on every pair
    // ...
    //  A quick sanity-check: run both the optimised ensemble and a
    //  naive equal-weight baseline over the ground-truth pairs so you
    //  can see the improvement at a glance.
    println!("\t|");
    println!(
        "\t| {:─<30} {:─<12} {:─<10} {:─<10} {:─<10}",
        "", "", "", "", ""
    );
    println!(
        "\t| {:<30} {:<12} {:<10} {:<10} {:<10}",
        "pair", "expected", "equal (0.5)", "optimised", "difference"
    );
    println!(
        "\t| {:─<30} {:─<12} {:─<10} {:─<10} {:─<10}",
        "", "", "", "", ""
    );

    let baseline = EnsembleAlgorithm {
        algorithms: vec![
            WeightedAlgorithm::new(levenshtein.clone(), 0.5),
            WeightedAlgorithm::new(jaro_winkler.clone(), 0.5),
        ],
        allow_negative_weights: false,
        is_probability_distribution: true,
    };

    for (a, b, expected) in &ground_truth {
        let label = format!("{a}/{b}");
        let base = baseline.similarity(a, b).unwrap_or(f32::NAN);
        let opt = ensemble.similarity(a, b).unwrap_or(f32::NAN);
        let delta = (opt - expected).abs() - (base - expected).abs();

        println!(
            "\t| {:<30} {:<12.4} {:<10.4} {:<10.4} {:+.4}",
            label, expected, base, opt, delta
        );
    }

    println!("\t|");
    println!("\t| Negative delta = optimised is closer to ground truth");
}
