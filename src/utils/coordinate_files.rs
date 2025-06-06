use serde::Deserialize;
use std::{fs::File, path::Path};

#[derive(Debug, Deserialize)]
pub struct CsvEntry {
    pub index: i64,
    pub longitude: f64,
    pub latitude: f64,
}

pub fn load_coordinate_file(file_path: &Path) -> Vec<CsvEntry> {
    let file = File::open(file_path).expect("Failed to open file");

    let mut coordinates: Vec<CsvEntry> = Vec::new();

    let mut serde_reader = csv::ReaderBuilder::new()
        .has_headers(true) // Specify that the file contains headers
        .from_reader(file);

    for result in serde_reader.deserialize() {
        let record: CsvEntry = result.expect("Failed to deserialize");
        coordinates.push(record);
    }

    coordinates
}
