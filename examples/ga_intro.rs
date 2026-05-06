use pho::{
    algorithms::{Algorithm, JaroWinkler, Levenshtein},
    dataset::{Dataset, Row},
    ensemble::{
        config::EnsembleConfig, types::EnsembleAlgorithm, weighted_function::WeightedFunction,
    },
    learning::{
        genetic::{GeneticConfig, optimize},
        loss::MeanSquaredError,
    },
    utils::io::import,
};

fn main() {
    println!("🍜\t| # tutorial: genetic algorithm introduction");
    // Genetic Algorithm
    // ...
    //  The ensemble has weights attached to each individual
    //  algorithm.
    //
    //  The next question is: "how do I get the best weights?"
    //
    //  This example gives one of the first techniques:
    //      **Genetic Algorithm**
    //

    // Load and validate configs
    let levenshtein = import::<Levenshtein>("tests/config_sample_levenshtein.toml").unwrap();
    let jaro_winkler = import::<JaroWinkler>("tests/config_sample_jaro_winkler.toml").unwrap();
    jaro_winkler.validate().unwrap();

    // Build a labelled dataset (either from CSV or inline fallback)
    let labeled_data: Vec<(String, String, f32)> = vec![
        ("dixon".into(), "dicksonx".into(), 0.81),
        ("martha".into(), "marhta".into(), 0.96),
        ("jellyfish".into(), "smellyfish".into(), 0.73),
        ("cat".into(), "car".into(), 0.83),
        ("saturday".into(), "sunday".into(), 0.62),
        ("bupropion".into(), "buspirone".into(), 0.76),
    ];

    // Build the ensemble with equal starting weights
    // ...
    //  The genetic algorithm will overwrite these weights, so their
    //  initial values don't matter as long as the ensemble is valid.
    //
    // Ensemble Configurations
    // ...
    //  An ensemble can take one of the four predefined configurations
    //      1. Linear: No limits on weights (can be negative, no sum
    //          requirement).
    //      2. Conical: Weights must be >= 0.0.
    //      3. Affine: Weights must sum to 1.0, but can be negative.
    //      4. Convex: Weights must sum to 1.0 and must be >= 0.0
    //          (Probability Distribution).
    //
    // Weighted Functions
    // ...
    //  There are multiple ways to add a WeightedAlgorithm to an
    //  ensemble: `from_function`: this takes in a user defined
    //  closure. And, `from_similarity`, `from_distance`, and
    //  `from_normalized_distance` takes in a pho defined algorithm
    //  and uses its similarity, distance, or normalized distance
    //  function (whichever one is picked).
    //
    //  And all of these function take in a f32 as a weight
    let mut ensemble: EnsembleAlgorithm = EnsembleAlgorithm::try_new(
        vec![
            WeightedFunction::from_similarity(levenshtein.clone(), 1.0),
            WeightedFunction::from_similarity(jaro_winkler.clone(), 1.0),
        ],
        EnsembleConfig::Linear,
    )
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
    //  `Dataset` from it.
    let training_rows = labeled_data
        .iter()
        .map(|(x, y, label)| Row::builder(x, y).label(*label).build())
        .collect::<Vec<_>>();
    let training_data = Dataset::from_ensemble(&ensemble, &training_rows, true).unwrap();

    // Saving a Dataset
    // ...
    //  We can save a dataset as a CSV or Arrow file by simply running
    //  the `.export()` on the `Dataset` variable.
    //
    //  The file type of the dataset (i.e., how to write it) is
    //  inferred from the extension. Hence, the example below will
    //  make a CSV file.
    training_data.export("sample_dataset.csv").unwrap();

    // Define the evaluator
    // ...
    //  Since these are **loss** functions, the genetic algorithm's
    //  goal is to minimize this.
    //
    //  This example uses `MeanSquaredError` but you can change the
    //  loss function.
    let evaluator = MeanSquaredError::new(&training_data);
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

    // Inspect the optimized weights
    let w_lev = ensemble.algorithms[0].weight;
    let w_jaro = ensemble.algorithms[1].weight;

    println!("\t|");
    println!("\t| Optimised weights:");
    println!("\t|   Levenshtein  : {w_lev:.4}");
    println!("\t|   Jaro-Winkler : {w_jaro:.4}");

    // Compare optimized vs. equal-weight ensemble on every pair
    // ...
    //  A quick sanity-check: run both the optimized ensemble and a
    //  naive equal-weight baseline over the ground-truth pairs so you
    //  can see the improvement at a glance.
    println!("\t|");
    println!(
        "\t| {:─<30} {:─<12} {:─<12} {:─<10} {:─<10}",
        "", "", "", "", ""
    );
    println!(
        "\t| {:<30} {:<12} {:<12} {:<10} {:<10}",
        "pair", "expected", "unoptimized", "optimized", "difference"
    );
    println!(
        "\t| {:─<30} {:─<12} {:─<12} {:─<10} {:─<10}",
        "", "", "", "", ""
    );

    let baseline: EnsembleAlgorithm = EnsembleAlgorithm::try_new(
        vec![
            WeightedFunction::from_similarity(levenshtein.clone(), 0.5),
            WeightedFunction::from_similarity(jaro_winkler.clone(), 0.5),
        ],
        EnsembleConfig::Convex,
    )
    .unwrap();

    for (a, b, expected) in labeled_data
        .iter()
        .map(|(a, b, t)| (a.as_str(), b.as_str(), *t))
    {
        let label = format!("{a}/{b}");
        let base = baseline.similarity(a, b).unwrap_or(f32::NAN);
        let opt = ensemble.similarity(a, b).unwrap_or(f32::NAN);
        let delta = (opt - expected).abs() - (base - expected).abs();

        println!(
            "\t| {:<30} {:<12.4} {:<12.4} {:<10.4} {:+.4}",
            label, expected, base, opt, delta
        );
    }

    println!("\t|");
    println!("\t| Negative delta = optimized is closer to ground truth");
}
