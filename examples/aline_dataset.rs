use pho::{
    Result,
    algorithms::Aline,
    dataset::{Dataset, Row},
    utils::io::{import, read_csv_as},
};

fn main() -> Result<()> {
    println!("🍜\t| # tutorial: ALINE dataset from CSV transcriptions");

    // Load ALINE configuration.
    let aline: Aline = import("tests/config_sample_aline.toml")?;

    // Read a CSV that explicitly carries transcriptions for both inputs.
    let rows: Vec<Row> = read_csv_as("examples/data/aline_dataset.csv", None)?;

    let dataset = Dataset::from_slice(vec![Box::new(aline)], &rows)?;

    dataset.export("example_dataset_aline.csv")?;
    dataset.export("example_dataset_aline.arrow")?;

    println!("\t| rows loaded: {}", dataset.inputs.len());
    println!("\t| Exported: example_dataset_aline.csv");
    println!("\t| Exported: example_dataset_aline.arrow");
    Ok(())
}
