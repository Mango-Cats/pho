use clap::{Args, Parser, Subcommand};
use include_dir::{Dir, include_dir};
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

/// Ready-made algorithm configs shipped in the repo, embedded into the
/// binary at compile time. Used as the default config source for the
/// single-pair subcommands (`orth`, `phon-nipa`, `phon-yipa`) so they work
/// out of the box without `--config-dir`, regardless of the current working
/// directory the binary happens to be run from (unlike a plain relative
/// path, which would only resolve when run from the repo root).
static DEFAULT_CONFIGS: Dir = include_dir!("$CARGO_MANIFEST_DIR/algorithm_configs/eng");

#[derive(Parser, Debug)]
#[command(
    name = "phoc",
    version,
    about = "Compute phonetic/orthographic similarity: batch-score a CSV, or run one pair immediately."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate a similarity feature CSV from input pairs (batch mode).
    Csv(CsvArgs),
    /// Run every plain orthographic (non-phonetic) string-similarity
    /// algorithm on one pair of spellings, e.g. `phoc orth hello hilo`.
    Orth(PairArgs),
    /// Run every orthography-based phonetic algorithm (phonetic algorithms
    /// that take plain spelling rather than IPA, e.g. Soundex, Metaphone) on
    /// one pair of spellings.
    #[command(name = "phon-nipa")]
    PhonNipa(PairArgs),
    /// Run every IPA-based phonetic algorithm (e.g. ALINE) on one pair of
    /// IPA transcriptions.
    #[command(name = "phon-yipa")]
    PhonYipa(PairArgs),
}

#[derive(Args, Debug)]
struct CsvArgs {
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
    /// Include Filipino (Tagalog) nativization indicator features
    /// (fil_vowel_skeleton_match, fil_penult_vowel_match, fil_onset_match,
    /// fil_coda_match, fil_phonetic_equal) in the output.
    #[arg(long)]
    include_fil_features: bool,
    /// Show a progress bar while computing scores.
    #[arg(long)]
    progress: bool,
}

#[derive(Args, Debug)]
struct PairArgs {
    /// First input string.
    x: String,
    /// Second input string.
    y: String,
    /// Directory containing algorithm config TOML files to draw this
    /// group's algorithms from. Defaults to the repo's ready-made configs,
    /// which are embedded in the binary at build time.
    #[arg(long)]
    config_dir: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Csv(args) => run_csv(args),
        Command::Orth(args) => run_pair(args, Group::Orth),
        Command::PhonNipa(args) => run_pair(args, Group::PhonNipa),
        Command::PhonYipa(args) => run_pair(args, Group::PhonYipa),
    }
}

fn run_csv(args: CsvArgs) -> Result<()> {
    let delimiter = parse_delimiter(&args.delimiter)?;

    // Read the input CSV keeping every original column. Only x_1/x_2/t_1/t_2/label
    // are used for scoring; all other columns are ignored for computation but
    // carried through verbatim to the output.
    let (headers, raw_rows, rows) = read_csv_records::<Row, _>(
        &args.input,
        Some(CSVOptions {
            delimiter,
            has_headers: !args.no_headers,
            flexible: args.flexible,
        }),
    )?;

    let algorithms = load_algorithms(&args.config_dir)?
        .into_iter()
        .map(|(name, _kind, algorithm)| (name, algorithm))
        .collect();
    let mut dataset =
        ScoreMatrix::from_named(algorithms, &rows, args.include_word_features, args.progress)?;
    if args.include_fil_features {
        dataset = dataset.with_fil_features()?;
    }
    let dataset = dataset.with_passthrough(headers, raw_rows)?;

    dataset.export(&args.output.to_string_lossy())?;
    Ok(())
}

/// Which of the three CLI-facing algorithm groups a config belongs to.
///
/// This is a presentation-layer grouping for `phoc`'s single-pair
/// subcommands, distinct from [`Algorithm::requires_transcription`] (which
/// only distinguishes IPA input from everything else): `PhonNipa` and `Orth`
/// both take plain spelling, but only `PhonNipa` algorithms are *designed*
/// to model pronunciation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Group {
    /// Plain orthographic / string-similarity algorithms: general string
    /// metrics, not designed to model pronunciation, even though they're
    /// commonly run on spelling here.
    Orth,
    /// Phonetic algorithms designed to run on plain spelling rather than
    /// IPA (Soundex, Metaphone, Double Metaphone, Editex, BI-SIM, Syllable,
    /// Needleman-Wunsch with its phonetic-confusability matrix).
    PhonNipa,
    /// Phonetic algorithms that require IPA transcriptions (ALINE).
    PhonYipa,
}

impl Group {
    fn of(kind: &str) -> Self {
        match kind {
            "aline" => Group::PhonYipa,
            "bisim" | "double_metaphone" | "doublemetaphone" | "editex" | "metaphone"
            | "needleman_wunsch" | "needlemanwunsch" | "soundex" | "syllable" => Group::PhonNipa,
            _ => Group::Orth,
        }
    }
}

/// Run every algorithm in `group` (loaded from `args.config_dir`, or the
/// embedded default configs when unset) on `(args.x, args.y)` and print
/// `name : score` for each, names left-aligned to the widest name so the
/// colons line up, sorted by name.
fn run_pair(args: PairArgs, group: Group) -> Result<()> {
    let algorithms = match &args.config_dir {
        Some(config_dir) => load_algorithms(config_dir)?,
        None => load_embedded_algorithms()?,
    };

    let mut results: Vec<(String, f32)> = algorithms
        .into_iter()
        .filter(|(_, kind, _)| Group::of(kind) == group)
        .map(|(name, _kind, algorithm)| {
            let score = algorithm.similarity(&args.x, &args.y)?;
            Ok((name, score))
        })
        .collect::<Result<_>>()?;
    results.sort_by(|a, b| a.0.cmp(&b.0));

    let width = results.iter().map(|(name, _)| name.len()).max().unwrap_or(0);
    for (name, score) in &results {
        println!("{name:width$} : {score:.4}");
    }
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
///
/// Insertions and deletions are directional: whichever of `x_1`/`x_2` a row
/// happens to put first decides which count is which, and rows carry no
/// canonical ordering. Reporting raw insertion/deletion counts as separate
/// columns would therefore make their difference (which encodes the signed
/// length delta between the two strings) an artifact of arbitrary pair
/// order rather than a real signal. `Indels` (their order-invariant sum)
/// and `IndelDiff` (the order-invariant absolute difference) carry the same
/// information — total indel operations and absolute length delta — without
/// the sign-arbitrary split. Substitutions are symmetric under swapping the
/// pair and need no such transform.
#[derive(Debug, Clone, Copy)]
enum EditOperation {
    Substitutions,
    Indels,
    IndelDiff,
}

impl EditOperation {
    const ALL: [Self; 3] = [Self::Substitutions, Self::Indels, Self::IndelDiff];

    fn suffix(self) -> &'static str {
        match self {
            Self::Substitutions => "substitutions",
            Self::Indels => "indels",
            Self::IndelDiff => "indel_diff",
        }
    }

    fn select(self, counts: (u32, u32, u32)) -> f32 {
        let (substitutions, insertions, deletions) = counts;
        match self {
            Self::Substitutions => substitutions as f32,
            Self::Indels => (insertions + deletions) as f32,
            Self::IndelDiff => (insertions as i64 - deletions as i64).unsigned_abs() as f32,
        }
    }
}

/// Adapts a `separate = true` algorithm's `edit_operation_counts` into a
/// single-column [`Algorithm`], so one config with the flag set fans out into
/// three columns (substitutions/indels/indel_diff) without changing how
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
/// yields three columns instead: `{stem}_substitutions`, `{stem}_indels`,
/// and `{stem}_indel_diff` — substitutions from the minimal-cost alignment,
/// the total insertion+deletion count, and the absolute difference between
/// insertions and deletions, rather than a single summed distance (see
/// [`EditOperation`] for why insertions/deletions aren't reported
/// separately). Files are processed in sorted order for deterministic
/// output.
///
/// This is also how "config-less" algorithms are included: to add `LCS`, drop
/// an `lcs.toml` containing just `algorithm = "lcs"` into the directory. There
/// is no implicit set of always-on algorithms — a column exists iff a file
/// asks for it.
///
/// Returns `(column_name, kind, algorithm)` triples; `kind` is the config's
/// `algorithm` key (used by [`Group::of`] to sort configs into `phoc`'s
/// single-pair subcommand groups).
fn load_algorithms(config_dir: &Path) -> Result<Vec<(String, String, Box<dyn Algorithm>)>> {
    let mut paths: Vec<PathBuf> = fs::read_dir(config_dir)?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("toml"))
        .collect();
    paths.sort();

    let sources = paths
        .into_iter()
        .map(|path| {
            let column_name = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or_default()
                .to_string();
            let label = path.display().to_string();
            let content = fs::read_to_string(&path)?;
            Ok((column_name, label, content))
        })
        .collect::<Result<Vec<_>>>()?;

    algorithms_from_sources(sources)
}

/// Same as [`load_algorithms`], but sourced from the ready-made configs
/// embedded in the binary (see [`DEFAULT_CONFIGS`]) instead of a directory
/// on disk.
fn load_embedded_algorithms() -> Result<Vec<(String, String, Box<dyn Algorithm>)>> {
    let mut files: Vec<_> = DEFAULT_CONFIGS
        .files()
        .filter(|file| file.path().extension().and_then(|ext| ext.to_str()) == Some("toml"))
        .collect();
    files.sort_by_key(|file| file.path());

    let sources = files
        .into_iter()
        .map(|file| {
            let column_name = file
                .path()
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or_default()
                .to_string();
            let label = format!("<embedded>/{}", file.path().display());
            let content = file.contents_utf8().ok_or_else(|| {
                Error::InvalidDatasetShape(format!("embedded config {label} is not valid UTF-8"))
            })?;
            Ok((column_name, label, content.to_string()))
        })
        .collect::<Result<Vec<_>>>()?;

    algorithms_from_sources(sources)
}

/// Build every algorithm from `(column_name, label, toml_content)` sources,
/// fanning `separate = true` configs out into three columns each (see
/// [`load_algorithms`]). `label` identifies the source in error messages
/// (a file path, or a marker for embedded configs).
fn algorithms_from_sources(
    sources: Vec<(String, String, String)>,
) -> Result<Vec<(String, String, Box<dyn Algorithm>)>> {
    let mut algorithms = Vec::with_capacity(sources.len());
    for (column_name, label, content) in sources {
        let (kind, algorithm) = build_algorithm(&label, &content)?;

        if algorithm.separate_enabled() {
            let shared: Arc<dyn Algorithm> = Arc::from(algorithm);
            for op in EditOperation::ALL {
                algorithms.push((
                    format!("{column_name}_{}", op.suffix()),
                    kind.clone(),
                    Box::new(EditOperationColumn {
                        inner: shared.clone(),
                        op,
                    }) as Box<dyn Algorithm>,
                ));
            }
        } else {
            algorithms.push((column_name, kind, algorithm));
        }
    }

    Ok(algorithms)
}

/// Build one algorithm from a config's TOML content, dispatching on its
/// `algorithm` key. `label` identifies the source for error messages.
///
/// The rest of the content is the algorithm's own config (the `algorithm`
/// key is ignored by the config structs). Config-less algorithms only need
/// the key. Returns the `algorithm` key alongside the built algorithm.
fn build_algorithm(label: &str, content: &str) -> Result<(String, Box<dyn Algorithm>)> {
    let table: toml::Table = toml::from_str(content)?;
    let kind = table
        .get("algorithm")
        .and_then(|value| value.as_str())
        .ok_or_else(|| Error::MissingAlgorithmKey {
            file: label.to_string(),
        })?
        .trim()
        .to_ascii_lowercase();

    // Deserialize the whole content into the selected config type. The
    // `algorithm` key is an unknown field to these structs and is silently
    // ignored.
    macro_rules! build {
        ($ty:ty) => {
            Box::new(toml::from_str::<$ty>(content)?) as Box<dyn Algorithm>
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
                file: label.to_string(),
            });
        }
    };

    Ok((kind, algorithm))
}
