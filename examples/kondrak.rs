use pho::{
    Result,
    algorithms::{Algorithm, Aline, BiSim, LCS, LCSuf},
    dataset::{Dataset, Row},
    utils::io::{import, read_csv_as},
};

type Pair = (String, String, f32);

fn kondrak_lasa_features() -> Result<Vec<Box<dyn Algorithm>>> {
    let aline: Aline = import("tests/config_sample_aline.toml")?;

    let bisim: BiSim = import("tests/config_sample_bisim.toml")?;

    // This basis set mirrors the Kondrak-oriented functions present in pho.
    Ok(vec![
        Box::new(aline),
        Box::new(bisim),
        Box::new(LCS::new(true)),
        Box::new(LCSuf::new(true)),
    ])
}

fn transcription_of(word: &str) -> String {
    // Ergonomic default for this tutorial: use the string itself as the
    // transcription carrier. In production, replace this with IPA lookup.
    word.to_string()
}

fn to_labeled_pairs(rows: &[Pair]) -> Vec<Row> {
    rows.iter()
        .map(|(x, y, target)| {
            Row::builder(x, y)
                .label(*target)
                .transcriptions(transcription_of(x), transcription_of(y))
                .build()
        })
        .collect()
}

fn keep_supported_rows(algorithms: &[Box<dyn Algorithm>], rows: &[Row]) -> Vec<Row> {
    rows.iter()
        .enumerate()
        .filter(|(_, row)| {
            algorithms.iter().all(|algorithm| {
                let left = if algorithm.requires_transcription() {
                    row.x_transcription.as_deref().unwrap_or("")
                } else {
                    row.x_1.as_str()
                };

                let right = if algorithm.requires_transcription() {
                    row.y_transcription.as_deref().unwrap_or("")
                } else {
                    row.x_2.as_str()
                };

                algorithm.similarity(left, right).is_ok()
            })
        })
        .map(|(_, row)| row.clone())
        .collect()
}

fn main() -> Result<()> {
    println!("🍜\t| # tutorial: Kondrak LASA-style dataset construction");

    // Assume `csv_data` exists as `Vec<(String, String, f32)>`.
    // For a runnable example we load it from disk.
    let csv_data: Vec<Pair> = read_csv_as("examples/data/dataset.csv", None)?;
    let labeled_pairs = to_labeled_pairs(&csv_data);

    let algorithms = kondrak_lasa_features()?;
    let filtered_rows = keep_supported_rows(&algorithms, &labeled_pairs);
    let dropped = labeled_pairs.len().saturating_sub(filtered_rows.len());

    if filtered_rows.is_empty() {
        println!("\t| No rows were compatible with all Kondrak LASA features.");
        return Ok(());
    }

    println!("\t| rows loaded   : {}", labeled_pairs.len());
    println!("\t| rows retained : {}", filtered_rows.len());
    println!("\t| rows dropped  : {dropped}");

    let dataset = Dataset::from_slice(algorithms, &filtered_rows)?;

    dataset.export("example_dataset_kondrak_lasa.csv")?;
    dataset.export("example_dataset_kondrak_lasa.arrow")?;

    println!("\t| Exported: example_dataset_kondrak_lasa.csv");
    println!("\t| Exported: example_dataset_kondrak_lasa.arrow");
    Ok(())
}
