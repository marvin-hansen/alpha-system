use crate::error::MDDBClientError;
use crate::MDDBClient;
use common_metadata::MetaAsset;
use proto_mddb::proto::*;
use proto_mddb_utils::*;

impl MDDBClient {
    /// Returns the total number of assets in the MDDB
    ///
    /// # Returns
    /// * `Result<u64, MDDBClientError>` - The count of assets on success, or an error if the operation fails
    ///
    pub async fn count_assets(&self) -> Result<u64, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_count_assets_request();

        match client.count_assets(request).await {
            Ok(res) => Ok(res.into_inner().count),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Checks if an asset with the given ID exists in the MDDB
    ///
    /// # Arguments
    /// * `asset_id` - The ID of the asset to check
    ///
    /// # Returns
    /// * `Result<bool, MDDBClientError>` - True if the asset exists, false otherwise, or an error if the operation fails
    ///
    pub async fn check_if_asset_id_exists(&self, asset_id: &str) -> Result<bool, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_check_if_asset_exists_request(asset_id);

        match client.check_if_asset_id_exists(request).await {
            Ok(res) => Ok(res.get_ref().exists),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves an asset from the MDDB by its ID
    ///
    /// # Arguments
    /// * `asset_id` - The ID of the asset to retrieve
    ///
    /// # Returns
    /// * `Result<Option<MetaAsset>, MDDBClientError>` - The asset if found, None if not found, or an error if the operation fails
    ///
    pub async fn get_asset(&self, asset_id: &str) -> Result<Option<MetaAsset>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_asset_request(asset_id);

        match client.get_asset(request).await {
            Ok(res) => Ok(res
                .into_inner()
                .asset
                .map(|asset| proto_asset_to_meta_asset(&asset))),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves all assets from the database and converts them to MetaAsset format.
    ///
    /// # Returns
    /// * `Result<Vec<MetaAsset>, MDDBClientError>` - A vector of MetaAsset objects if successful,
    ///   or an MDDBClientError if the operation fails
    ///
    pub async fn get_all_assets(&self) -> Result<Vec<MetaAsset>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_all_assets_request();

        match client.get_all_assets(request).await {
            Ok(res) => {
                let assets = res
                    .into_inner()
                    .assets
                    .into_iter()
                    .map(|proto_asset: ProtoMetaAsset| proto_asset_to_meta_asset(&proto_asset))
                    .collect();

                Ok(assets)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }
}
