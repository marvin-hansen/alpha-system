use crate::db::metadata::Metadata;
use crate::error::ClickHouseUtilError;
use common::prelude::Asset;

impl Metadata {
    pub async fn import_asset_metadata(
        &self,
        assets: &Vec<Asset>,
    ) -> Result<(), ClickHouseUtilError> {
        for asset in assets.iter() {
            let insert_query = self.generate_asset_insert(asset);
            self.execute_query(&insert_query)
                .await
                .expect("Failed to insert asset")
        }

        Ok(())
    }
}
