use crate::error::ClickHouseUtilError;
use crate::types::{Asset, AssetRoot};
use crate::ClickhouseUtil;
use std::fs::File;
use std::path::PathBuf;

impl ClickhouseUtil {
    pub async fn import_asset_data(&self, path: &str) -> Result<(), ClickHouseUtilError> {
        let assets = self
            .load_assets(path)
            .await
            .expect("Failed to load assets.json file");

        for asset in assets.iter() {
            let insert_query = self.metadata.generate_asset_insert(asset);
            self.execute_query(&insert_query)
                .await
                .expect("Failed to insert asset")
        }

        Ok(())
    }

    async fn load_assets(&self, path: &str) -> Result<Vec<Asset>, ClickHouseUtilError> {
        let file_path = PathBuf::from(path);
        let file = File::open(file_path).expect("file not found");
        let assets: AssetRoot = serde_json::from_reader(file).expect("error while reading");
        Ok(assets.data)
    }
}
