use db_utils::types::{Asset, AssetRoot};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub(crate) async fn load_assets(file_path: &PathBuf) -> Result<Vec<Asset>, Box<dyn Error>> {
    let file = File::open(file_path).expect("file not found");
    let assets: AssetRoot = serde_json::from_reader(file).expect("error while reading");
    Ok(assets.data)
}
