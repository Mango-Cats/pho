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

fn load_config<T>(dir: &Path, file_name: &str) -> Result<Option<T>>
where
    T: DeserializeOwned,
{
    let path = dir.join(file_name);
    match import(path.to_string_lossy().as_ref()) {
        Ok(config) => Ok(Some(config)),
        Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

macro_rules! push_if_present {
    ($algorithms:expr, $config_dir:expr, $type:ty, $file:expr) => {
        if let Some(cfg) = load_config::<$type>($config_dir, $file)? {
            $algorithms.push(Box::new(cfg) as Box<dyn Algorithm>);
        }
    };
}

fn load_algorithms(config_dir: &Path) -> Result<Vec<Box<dyn Algorithm>>> {
    let mut algorithms: Vec<Box<dyn Algorithm>> = Vec::new();

    push_if_present!(algorithms, config_dir, Aline, "aline.toml");
    push_if_present!(algorithms, config_dir, BiSim, "bisim.toml");
    push_if_present!(algorithms, config_dir, DoubleMetaphone, "double_metaphone.toml");
    push_if_present!(algorithms, config_dir, Editex, "editex.toml");
    push_if_present!(algorithms, config_dir, JaroWinkler, "jaro_winkler.toml");
    push_if_present!(algorithms, config_dir, Keyboard, "keyboard.toml");
    algorithms.push(Box::new(LCS::default()));
    algorithms.push(Box::new(LCSuf::default()));
    push_if_present!(algorithms, config_dir, Levenshtein, "levenshtein.toml");
    push_if_present!(algorithms, config_dir, Metaphone, "metaphone.toml");
    push_if_present!(algorithms, config_dir, NeedlemanWunsch, "needleman_wunsch.toml");
    push_if_present!(algorithms, config_dir, NGram, "ngram.toml");
    push_if_present!(algorithms, config_dir, Prefix, "prefix.toml");
    push_if_present!(algorithms, config_dir, SmithWaterman, "smith_waterman.toml");
    push_if_present!(algorithms, config_dir, Soundex, "soundex.toml");
    push_if_present!(algorithms, config_dir, Syllable, "syllable.toml");
    push_if_present!(algorithms, config_dir, CharTfIdf, "tfidf.toml");

    Ok(algorithms)
}
