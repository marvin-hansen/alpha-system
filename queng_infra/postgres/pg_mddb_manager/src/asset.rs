use crate::PostgresMDDBManager;
use common_errors::PostgresDBError;
use common_metadata::MetaAsset;
use pg_mddb::Asset;

impl PostgresMDDBManager {
    /// Inserts a new asset into the database.
    ///
    /// # Arguments
    ///
    /// * `asset` - A `MetaAsset` object representing the asset to be inserted.
    ///
    /// # Returns
    ///
    /// * `Result<MetaAsset, PostgresDBError>`
    ///
    /// Returns the inserted `MetaAsset` on success, or a `PostgresDBError` if the insertion fails.
    ///
    pub async fn insert_asset(&self, asset: MetaAsset) -> Result<MetaAsset, PostgresDBError> {
        self.dbg_print("insert_assets");
        let conn = &mut self.get_connection();

        match Asset::create_asset(conn, asset) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    /// Inserts a collection of assets into the database.
    ///
    /// # Arguments
    ///
    /// * `assets` - A slice of `MetaAsset` objects representing the assets to be inserted.
    ///
    /// # Returns
    ///
    /// * `Result<usize, PostgresDBError>`
    ///
    /// Returns the number of assets successfully inserted, or a `PostgresDBError` if the insertion fails.
    ///
    pub async fn insert_asset_collection(
        &self,
        assets: &[MetaAsset],
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("insert_asset_collection");
        let conn = &mut self.get_connection();

        match Asset::create_asset_collection(conn, assets) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    /// Counts the total number of assets in the database.
    ///
    /// # Returns
    ///
    /// * `Result<u64, PostgresDBError>`
    ///
    /// Returns the count of assets on success, or a `PostgresDBError` if the count operation fails.
    ///
    pub async fn count_assets(&self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_assets");
        let conn = &mut self.get_connection();

        match Asset::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
        }
    }

    /// Checks if an asset with the specified ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - A `String` representing the ID of the asset to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, PostgresDBError>`
    ///
    /// Returns `true` if the asset exists, `false` otherwise,
    ///   or a `PostgresDBError` if the check operation fails.
    ///
    pub async fn check_if_asset_id_exists(
        &self,
        asset_id: String,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_asset_id_exists");
        let conn = &mut self.get_connection();

        match Asset::check_if_asset_id_exists(conn, asset_id) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        }
    }

    /// Reads an asset with the specified ID from the database.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - A `String` representing the ID of the asset to read.
    ///
    /// # Returns
    ///
    /// * `Result<MetaAsset, PostgresDBError>`
    ///
    /// Returns the `MetaAsset` on success,
    ///   or a `PostgresDBError` if the read operation fails.
    ///
    pub async fn read_asset(&self, asset_id: String) -> Result<Option<MetaAsset>, PostgresDBError> {
        self.dbg_print("read_asset");
        let conn = &mut self.get_connection();

        match Asset::read(conn, asset_id) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Reads all assets from the database.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<MetaAsset>, PostgresDBError>`
    ///
    /// Returns a vector of `MetaAsset` objects on success, or a `PostgresDBError` if the read operation fails.
    pub async fn read_all_assets(&self) -> Result<Vec<MetaAsset>, PostgresDBError> {
        self.dbg_print("read_all_assets");
        let conn = &mut self.get_connection();

        match Asset::read_all(conn) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Updates an existing asset in the database with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - A `String` representing the ID of the asset to update.
    /// * `asset` - A `MetaAsset` object representing the updated asset data.
    ///
    /// # Returns
    ///
    /// * `Result<usize, PostgresDBError>`
    ///
    /// Returns the number of rows affected on success, or a `PostgresDBError` if the update operation fails.
    pub async fn update_asset(
        &self,
        asset_id: String,
        asset: MetaAsset,
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("update_asset");
        let conn = &mut self.get_connection();

        match Asset::update(conn, asset_id, asset) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::UpdateFailed(e.to_string())),
        }
    }

    /// Deletes an asset with the specified ID from the database.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - A `String` representing the ID of the asset to delete.
    ///
    /// # Returns
    ///
    /// * `Result<usize, PostgresDBError>`
    ///
    /// Returns the number of rows affected on success,
    ///   or a `PostgresDBError` if the delete operation fails.
    ///
    pub async fn delete_asset(&self, asset_id: String) -> Result<usize, PostgresDBError> {
        self.dbg_print("delete_asset");
        let conn = &mut self.get_connection();

        match Asset::delete(conn, asset_id) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::DeleteFailed(e.to_string())),
        }
    }
}
