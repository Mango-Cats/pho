use clap::Parser;
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};

use pho::algorithms::{
    Algorithm, Aline, BiSim, CharTfIdf, DoubleMetaphone, Editex, JaroWinkler, Keyboard, LCS, LCSuf,
    Levenshtein, Metaphone, NGram, NeedlemanWunsch, Prefix, SmithWaterman, Soundex, Syllable,
};
use pho::dataset::{Row, ScoreMatrix};
use pho::utils::io::{CSVOptions, import, read_csv_as};
use pho::{Error, Result};

#[derive(Parser, Debug)]
#[command(
    name = "phoc",
    version,
    about = "Generate a similarity feature CSV from input pairs."
)]
struct Cli {
    /// Input CSV file containing x_1/x_2 columns (optional: label, t_1, t_2).
    #[arg(short, long)]
    input: PathBuf,
    /// Output CSV file to write scored features.
    #[arg(short, long)]
    output: PathBuf,
    /// Directory containing algorithm config TOML files.
    #[arg(long)]
    config_dir: PathBuf,
    /// CSV field delimiter (single byte).
    #[arg(long, default_value = ",")]
    delimiter: String,
    /// Set when the input CSV has no header row.
    #[arg(long)]
    no_headers: bool,
    /// Allow variable-length rows in the input CSV.
    #[arg(long)]
    flexible: bool,
    /// Include word-level length features in the output.
    #[arg(long)]
    include_word_features: bool,
    /// Show a progress bar while computing scores.
    #[arg(long)]
    progress: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let delimiter = parse_delimiter(&cli.delimiter)?;

    let rows = read_csv_as::<Row, _>(
        &cli.input,
        Some(CSVOptions {
            delimiter,
            has_headers: !cli.no_headers,
            flexible: cli.flexible,
        }),
    )?;

    let algorithms = load_algorithms(&cli.config_dir)?;
    let dataset =
        ScoreMatrix::from_slice(algorithms, &rows, cli.include_word_features, cli.progress)?;

    dataset.export(&cli.output.to_string_lossy())?;
    Ok(())
}

fn parse_delimiter(delimiter: &str) -> Result<u8> {
    let bytes = delimiter.as_bytes();
    if bytes.len() != 1 {
        return Err(Error::InvalidDatasetShape(
            "delimiter must be a single byte".to_string(),
        ));
    }

    Ok(bytes[0])
}

fn load_config<T>(dir: &Path, file_name: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let path = dir.join(file_name);
    import(path.to_string_lossy().as_ref())
}

fn load_algorithms(config_dir: &Path) -> Result<Vec<Box<dyn Algorithm>>> {
    let mut algorithms: Vec<Box<dyn Algorithm>> = Vec::new();

    algorithms.push(Box::new(load_config::<Aline>(config_dir, "aline.toml")?));
    algorithms.push(Box::new(load_config::<BiSim>(config_dir, "bisim.toml")?));
    algorithms.push(Box::new(load_config::<DoubleMetaphone>(
        config_dir,
        "double_metaphone.toml",
    )?));
    algorithms.push(Box::new(load_config::<Editex>(config_dir, "editex.toml")?));
    algorithms.push(Box::new(load_config::<JaroWinkler>(
        config_dir,
        "jaro_winkler.toml",
    )?));
    algorithms.push(Box::new(load_config::<Keyboard>(
        config_dir,
        "keyboard.toml",
    )?));
    algorithms.push(Box::new(LCS::default()));
    algorithms.push(Box::new(LCSuf::default()));
    algorithms.push(Box::new(load_config::<Levenshtein>(
        config_dir,
        "levenshtein.toml",
    )?));
    algorithms.push(Box::new(load_config::<Metaphone>(
        config_dir,
        "metaphone.toml",
    )?));
    algorithms.push(Box::new(load_config::<NeedlemanWunsch>(
        config_dir,
        "needleman_wunsch.toml",
    )?));
    algorithms.push(Box::new(load_config::<NGram>(config_dir, "ngram.toml")?));
    algorithms.push(Box::new(load_config::<Prefix>(config_dir, "prefix.toml")?));
    algorithms.push(Box::new(load_config::<SmithWaterman>(
        config_dir,
        "smith_waterman.toml",
    )?));
    algorithms.push(Box::new(load_config::<Soundex>(
        config_dir,
        "soundex.toml",
    )?));
    algorithms.push(Box::new(load_config::<Syllable>(
        config_dir,
        "syllable.toml",
    )?));
    algorithms.push(Box::new(load_config::<CharTfIdf>(
        config_dir,
        "tfidf.toml",
    )?));

    Ok(algorithms)
}
