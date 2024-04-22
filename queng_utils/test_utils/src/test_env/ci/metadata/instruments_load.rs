use db_utils::types::{Instrument, InstrumentsRoot};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub(crate) async fn load_instruments(
    file_path: &PathBuf,
) -> Result<Vec<Instrument>, Box<dyn Error>> {
    let file = File::open(file_path).expect("instruments.json file not found");
    let instruments: InstrumentsRoot = serde_json::from_reader(file).expect("error while reading");
    Ok(instruments.data)
}
