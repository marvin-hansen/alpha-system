use crate::db::metadata::Metadata;
use crate::types::error::ClickHouseUtilError;
use common::prelude::Asset;

impl Metadata {
    /// Imports asset metadata into the metadata database.
    ///
    /// This method imports asset metadata into the metadata database.
    /// It takes a reference to a vector of `Asset` objects, and iterates over each asset.
    /// For each asset, it generates an SQL insert query using the `generate_asset_insert` method.
    /// The generated query is then executed using the `execute_query` method.
    ///
    /// # Arguments
    ///
    /// * `&self` - A reference to the `Metadata` object.
    /// * `assets` - A reference to a vector of `Asset` objects.
    ///
    /// # Returns
    ///
    /// * `Result<(), ClickHouseUtilError>` - The result of executing the query. Returns `Ok(())` if the metadata is imported successfully, or an `Err` containing the error if the import fails.
    ///
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
