use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use pho::algorithms::{
    Algorithm, Aline, BiSim, CharTfIdf, DoubleMetaphone, Editex, JaroWinkler, Keyboard, LCS,
    LCSubstring, LCSuf, Levenshtein, Metaphone, NGram, NeedlemanWunsch, Prefix, SmithWaterman,
    Soundex, Syllable, VisualWeighted,
};
use pho::dataset::{Row, ScoreMatrix};
use pho::utils::io::{CSVOptions, read_csv_records};
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
    /// Directory containing algorithm config TOML files. Each `.toml` file
    /// produces one feature column named after the file (or three, if the
    /// file sets `separate = true`); the file's `algorithm` key selects which
    /// algorithm computes it.
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

    // Read the input CSV keeping every original column. Only x_1/x_2/t_1/t_2/label
    // are used for scoring; all other columns are ignored for computation but
    // carried through verbatim to the output.
    let (headers, raw_rows, rows) = read_csv_records::<Row, _>(
        &cli.input,
        Some(CSVOptions {
            delimiter,
            has_headers: !cli.no_headers,
            flexible: cli.flexible,
        }),
    )?;

    let algorithms = load_algorithms(&cli.config_dir)?;
    let dataset =
        ScoreMatrix::from_named(algorithms, &rows, cli.include_word_features, cli.progress)?
            .with_passthrough(headers, raw_rows)?;

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

/// Which operation tally an [`EditOperationColumn`] reports.
#[derive(Debug, Clone, Copy)]
enum EditOperation {
    Substitutions,
    Insertions,
    Deletions,
}

impl EditOperation {
    const ALL: [Self; 3] = [Self::Substitutions, Self::Insertions, Self::Deletions];

    fn suffix(self) -> &'static str {
        match self {
            Self::Substitutions => "substitutions",
            Self::Insertions => "insertions",
            Self::Deletions => "deletions",
        }
    }

    fn select(self, counts: (u32, u32, u32)) -> f32 {
        let (substitutions, insertions, deletions) = counts;
        match self {
            Self::Substitutions => substitutions as f32,
            Self::Insertions => insertions as f32,
            Self::Deletions => deletions as f32,
        }
    }
}

/// Adapts a `separate = true` algorithm's `edit_operation_counts` into a
/// single-column [`Algorithm`], so one config with the flag set fans out into
/// three columns (substitutions/insertions/deletions) without changing how
/// [`ScoreMatrix`] scores columns.
struct EditOperationColumn {
    inner: Arc<dyn Algorithm>,
    op: EditOperation,
}

impl Algorithm for EditOperationColumn {
    fn similarity(&self, x: &str, y: &str) -> Result<f32> {
        let counts = self.inner.edit_operation_counts(x, y)?;
        Ok(self.op.select(counts))
    }

    fn requires_transcription(&self) -> bool {
        self.inner.requires_transcription()
    }

    fn name(&self) -> &'static str {
        self.inner.name()
    }
}

/// Load one algorithm per `.toml` file in `config_dir`.
///
/// Each file yields one `(column_name, algorithm)` pair where the column name
/// is the file stem (so `my_sim.toml` produces a `my_sim` column) and the
/// algorithm is selected by the file's required `algorithm` key — *unless*
/// the config sets `separate = true` (only meaningful for edit-distance
/// algorithms that implement `edit_operation_counts`), in which case it
/// yields three columns instead: `{stem}_substitutions`, `{stem}_insertions`,
/// and `{stem}_deletions`, each reporting the literal operation tally from
/// the minimal-cost alignment rather than a single summed distance. Files are
/// processed in sorted order for deterministic output.
///
/// This is also how "config-less" algorithms are included: to add `LCS`, drop
/// an `lcs.toml` containing just `algorithm = "lcs"` into the directory. There
/// is no implicit set of always-on algorithms — a column exists iff a file
/// asks for it.
fn load_algorithms(config_dir: &Path) -> Result<Vec<(String, Box<dyn Algorithm>)>> {
    let mut paths: Vec<PathBuf> = fs::read_dir(config_dir)?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("toml"))
        .collect();
    paths.sort();

    let mut algorithms = Vec::with_capacity(paths.len());
    for path in paths {
        let column_name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_default()
            .to_string();
        let algorithm = build_algorithm(&path)?;

        if algorithm.separate_enabled() {
            let shared: Arc<dyn Algorithm> = Arc::from(algorithm);
            for op in EditOperation::ALL {
                algorithms.push((
                    format!("{column_name}_{}", op.suffix()),
                    Box::new(EditOperationColumn {
                        inner: shared.clone(),
                        op,
                    }) as Box<dyn Algorithm>,
                ));
            }
        } else {
            algorithms.push((column_name, algorithm));
        }
    }

    Ok(algorithms)
}

/// Build one algorithm from a config file, dispatching on its `algorithm` key.
///
/// The rest of the file is the algorithm's own config (the `algorithm` key is
/// ignored by the config structs). Config-less algorithms only need the key.
fn build_algorithm(path: &Path) -> Result<Box<dyn Algorithm>> {
    let content = fs::read_to_string(path)?;
    let table: toml::Table = toml::from_str(&content)?;
    let kind = table
        .get("algorithm")
        .and_then(|value| value.as_str())
        .ok_or_else(|| Error::MissingAlgorithmKey {
            file: path.display().to_string(),
        })?
        .trim()
        .to_ascii_lowercase();

    // Deserialize the whole file into the selected config type. The `algorithm`
    // key is an unknown field to these structs and is silently ignored.
    macro_rules! build {
        ($ty:ty) => {
            Box::new(toml::from_str::<$ty>(&content)?) as Box<dyn Algorithm>
        };
    }

    let algorithm = match kind.as_str() {
        "aline" => build!(Aline),
        "bisim" => build!(BiSim),
        "double_metaphone" | "doublemetaphone" => build!(DoubleMetaphone),
        "editex" => build!(Editex),
        "jaro_winkler" | "jarowinkler" => build!(JaroWinkler),
        "keyboard" => build!(Keyboard),
        "lcs" => build!(LCS),
        "lcsubstring" => build!(LCSubstring),
        "lcsuf" => build!(LCSuf),
        "levenshtein" => build!(Levenshtein),
        "metaphone" => build!(Metaphone),
        "needleman_wunsch" | "needlemanwunsch" => build!(NeedlemanWunsch),
        "ngram" => build!(NGram),
        "prefix" => build!(Prefix),
        "smith_waterman" | "smithwaterman" => build!(SmithWaterman),
        "soundex" => build!(Soundex),
        "syllable" => build!(Syllable),
        "tfidf" | "chartfidf" => build!(CharTfIdf),
        "visual_weighted" | "visualweighted" => build!(VisualWeighted),
        other => {
            return Err(Error::UnknownAlgorithm {
                algorithm: other.to_string(),
                file: path.display().to_string(),
            });
        }
    };

    Ok(algorithm)
}
