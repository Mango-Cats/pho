use pho::{
    algorithms::{Algorithm, Aline},
    utils::io::import,
};
use std::error::Error;

const ENG_TOML_PATH: &str = "algorithm_configs/eng/aline.toml";
const FIL_TOML_PATH: &str = "algorithm_configs/fil/filipino_aline.toml";

// const EVAL_SET: &str = "algorithm_configs/fil/data/eval.csv";
const EVAL_SET: &str = "algorithm_configs/fil/data/drugs.csv";

#[derive(Debug, serde::Deserialize)]
struct Row {
    a: String,
    b: String,
    language: String,
}

#[derive(Debug)]
struct Pair {
    a: String, 
    b: String,
    language: String, 
    eng_score: f32,
    fil_score: f32
}

fn main() -> Result<(), Box<dyn Error>> {
    let eng_config = load_eng();
    let fil_config = load_fil();

    let mut pairs = Vec::new();
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path(EVAL_SET)?;

    for result in rdr.deserialize() {
        let row: Row = result?;
        let eng_score = eng_config.similarity(&row.a, &row.b).unwrap();
        let fil_score = fil_config.similarity(&row.a, &row.b).unwrap();

        pairs.push(Pair {
            a: row.a,
            b: row.b,
            language: row.language,
            eng_score,
            fil_score,
        });

        // println!("{:?}", result);
    }

    print_scores(&pairs);

    Ok(())
}

fn load_eng() -> Aline {
    match import(ENG_TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {ENG_TOML_PATH}: {e}."),
    }
}

fn load_fil() -> Aline {
    match import(FIL_TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {FIL_TOML_PATH}: {e}."),
    }
}

enum Outcome {
    Pass,
    Fail,
    Tie,
}

// i used claude to help format this lol
fn print_scores(pairs: &[Pair]) {
    let mut eng_passes = 0;
    let mut eng_fails = 0;
    let mut fil_passes = 0;
    let mut fil_fails = 0;
    let mut eng_tie = 0;
    let mut fil_tie = 0;

    println!(
        "{:<12} {:<12} {:<10} {:>12} {:>12} {:>12} {}",
        "A", "B", "Language", "EngScore", "FilScore", "Difference", "Result"
    );
    println!("{}", "-".repeat(70));

    for p in pairs {
        let outcome = if p.eng_score == p.fil_score {
            Outcome::Tie
        } else if (p.eng_score > p.fil_score && p.language == "english")
            || (p.fil_score > p.eng_score && p.language == "filipino")
        {
            Outcome::Pass
        } else {
            Outcome::Fail
        };

         match (p.language.as_str(), &outcome) {
            ("english", Outcome::Pass) => eng_passes += 1,
            ("english", Outcome::Fail) => eng_fails += 1,
            ("english", Outcome::Tie) => eng_tie += 1,
            ("filipino", Outcome::Pass) => fil_passes += 1,
            ("filipino", Outcome::Fail) => fil_fails += 1,
            ("filipino", Outcome::Tie) => fil_tie += 1,
            _ => {}
        }

        let result = match outcome {
            Outcome::Pass => "✓",
            Outcome::Fail => "x",
            Outcome::Tie => "●",
        };

        let difference = (p.eng_score - p.fil_score).abs();
        println!(
            "{:<12} {:<12} {:<10} {:>12} {:>12} {:>12} {:>4}",
            p.a, p.b, p.language, p.eng_score, p.fil_score, difference, result
        );
    }

    println!("{}", "-".repeat(70));

    println!("English > Passed: {eng_passes}   Failed: {eng_fails}  Ties: {eng_tie}  Total: {}", eng_passes + eng_fails +eng_tie);
    println!("Filipino > Passed: {fil_passes}   Failed: {fil_fails}  Ties: {fil_tie}  Total: {}", fil_passes + fil_fails + fil_tie);
    println!(
        "Overall > Passed: {}   Failed: {}  Ties: {}  Total: {}",
        eng_passes + fil_passes,
        eng_fails + fil_fails,
        eng_tie + fil_tie,
        pairs.len()
    );
}
