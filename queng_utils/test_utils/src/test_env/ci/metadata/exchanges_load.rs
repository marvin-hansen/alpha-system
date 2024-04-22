use db_utils::types::{Exchange, ExchangesRoot};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub(crate) async fn load_exchanges(file_path: &PathBuf) -> Result<Vec<Exchange>, Box<dyn Error>> {
    let file = File::open(file_path).expect("file not found");
    let exchanges: ExchangesRoot = serde_json::from_reader(file).expect("error while reading");
    Ok(exchanges.data)
}
