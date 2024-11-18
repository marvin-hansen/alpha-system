use crate::model::asset::Asset;
use crate::prelude::UpdateAsset;
use crate::schema::mddb::assets::asset_code;
use crate::schema::mddb::assets::dsl::assets as assets_table;
use crate::Connection;
use common_metadata::MetaAsset;
use diesel::dsl::insert_into;
use diesel::ExpressionMethods;
use diesel::{OptionalExtension, QueryDsl, QueryResult, RunQueryDsl};

impl Asset {
    /// Create a new asset in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `meta_asset` - The metadata of the asset to be created.
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult` containing the created `MetaAsset` if successful, or an error.
    ///
    pub fn create_asset(db: &mut Connection, meta_asset: MetaAsset) -> QueryResult<MetaAsset> {
        let asset = Asset::from_meta_asset(meta_asset);
        match insert_into(assets_table)
            .values(&asset)
            .get_result::<Asset>(db)
        {
            Ok(asset) => Ok(asset.to_meta_asset()),
            Err(e) => Err(e),
        }
    }

    /// Insert a collection of assets into the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `meta_assets` - A slice of `MetaAsset` to be inserted.
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult` indicating success or failure of the operation.
    ///
    pub fn create_asset_collection(
        conn: &mut Connection,
        meta_assets: &[MetaAsset],
    ) -> QueryResult<usize> {
        let items: Vec<Asset> = meta_assets
            .iter()
            .map(|ma| Asset::from_meta_asset(ma.clone()))
            .collect();
        match insert_into(assets_table).values(&items).execute(conn) {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    /// Count the total number of assets in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult` containing the count of assets as `u64` if successful, or an error.
    ///
    pub fn count(db: &mut Connection) -> QueryResult<u64> {
        assets_table.count().get_result::<i64>(db).map(|c| c as u64)
    }

    /// Check if an asset with the given asset ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `asset_id` - The ID of the asset to check for existence.
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult` indicating whether the asset exists or not.
    ///
    pub fn check_if_asset_id_exists(db: &mut Connection, asset_id: String) -> QueryResult<bool> {
        let exists = assets_table
            .find(asset_id)
            .first::<Asset>(db)
            .optional()?
            .is_some();
        Ok(exists)
    }

    /// Read an asset from the database by asset ID.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `asset_id` - The ID of the asset to read.
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult` containing the retrieved `MetaAsset` if successful, or an error.
    ///
    pub fn read(db: &mut Connection, param_asset_id: String) -> QueryResult<Option<MetaAsset>> {
        let exists = Self::check_if_asset_id_exists(db, param_asset_id.clone())?;

        if !exists {
            Ok(None)
        } else {
            assets_table
                .filter(asset_code.eq(param_asset_id))
                .first::<Asset>(db)
                .map(|a| Some(a.to_meta_asset()))
        }
    }

    /// Read all assets from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult` containing a vector of `MetaAsset` if successful, or an error.
    ///
    pub fn read_all(db: &mut Connection) -> QueryResult<Vec<MetaAsset>> {
        assets_table
            .load::<Asset>(db)
            .map(|a| a.into_iter().map(|asset| asset.to_meta_asset()).collect())
    }

    /// Update an asset in the database by asset ID.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `asset_id` - The ID of the asset to update.
    /// * `item` - The `UpdateAsset` containing the fields to update.
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult` containing the number of rows affected by the update operation.
    ///
    pub fn update(
        db: &mut Connection,
        asset_id: String,
        meta_asset: MetaAsset,
    ) -> QueryResult<usize> {
        let item = UpdateAsset::from_meta_asset(meta_asset);
        diesel::update(assets_table.find(asset_id))
            .set(&item)
            .execute(db)
    }

    /// Delete an asset from the database by asset ID.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `asset_id` - The ID of the asset to delete.
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult` containing the number of rows affected by the delete operation.
    /// If the asset does not exist, the query will return `Ok(0)`.
    /// If the asset exists and was deleted, the query will return `Ok(1)`.
    ///
    /// Note, delete only returns an error when either the database connection or the query fails.
    ///
    pub fn delete(db: &mut Connection, asset_id: String) -> QueryResult<usize> {
        diesel::delete(assets_table.find(asset_id)).execute(db)
    }
}
