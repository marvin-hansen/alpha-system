use crate::db::metadata::{Metadata, ASSETS_TABLE, DB_NAME};
use crate::types::error::ClickHouseUtilError;
use common::prelude::{Asset, AssetMetadata};

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

    /// Generates a ClickHouse SQL query to insert an asset into the assets table.
    ///
    /// This method generates a SQL query that can be used to insert an asset into the assets table.
    ///
    /// # Arguments
    ///
    /// * `asset` - A reference to an `Asset` object that contains the asset data to be inserted.
    ///
    /// # Returns
    ///
    /// * `String` - The generated SQL query.
    ///
    pub(crate) fn generate_asset_insert(&self, asset: &Asset) -> String {
        let table_name = format!("{DB_NAME}.{ASSETS_TABLE}");
        let code = asset.code.clone();
        // ClickHouse needs quotes to be escaped
        // https://github.com/ClickHouse/ClickHouse/issues/191
        let name = asset.name.replace('\'', "\\'");
        let asset_class = &asset.asset_class;
        let asset_figi = self.extract_asset_figi(&asset.metadata);

        format!(
            r"
        INSERT INTO {table_name} (*)
        VALUES (
        '{code}',
        '{name}',
        '{asset_class}',
        '{asset_figi}');"
        )
    }

    fn extract_asset_figi(&self, metadata: &Option<AssetMetadata>) -> String {
        let empty_string = "".to_string();

        match metadata {
            Some(metadata) => match &metadata.asset_figi {
                Some(figi) => figi.to_owned(),
                None => empty_string,
            },
            None => empty_string,
        }
    }
}
