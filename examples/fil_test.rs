use pho::{
    algorithms::{Algorithm, Aline},
    utils::io::import,
};
use std::error::Error;

const ENG_TOML_PATH: &str = "algorithm_configs/eng/aline.toml";
const FIL_TOML_PATH: &str = "algorithm_configs/fil/filipino_aline.toml";
const EVAL_SET: &str = "algorithm_configs/fil/data/eval.csv";

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

// i used claude to help format this lol
fn print_scores(pairs: &[Pair]) {
    let mut pass = 0;
    let mut fail = 0;

    println!(
        "{:<12} {:<12} {:<10} {:>12} {:>12}  {}",
        "A", "B", "Language", "EngScore", "FilScore", "Result"
    );
    println!("{}", "-".repeat(70));

    for p in pairs {
        let passed = (p.eng_score > p.fil_score && p.language == "english")
            || (p.fil_score > p.eng_score && p.language == "filipino");

        if passed {
            pass += 1;
        } else {
            fail += 1;
        }

        let result = if passed { "✓" } else { "x" };

        println!(
            "{:<12} {:<12} {:<10} {:>12} {:>12}  {:>4}",
            p.a, p.b, p.language, p.eng_score, p.fil_score, result
        );
    }

    println!("{}", "-".repeat(70));
    println!("Passed: {pass}   Failed: {fail}   Total: {}", pass + fail);
}
