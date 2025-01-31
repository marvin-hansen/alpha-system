/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::model::asset::Asset;
use crate::schema::mddb::assets::asset_code;
use crate::schema::mddb::assets::dsl::assets as assets_table;
use crate::Connection;
use crate::UpdateAsset;
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
    /// Returns a `QueryResult` containing the created `MetaAsset` if successful.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Unique constraint violation if an asset with the same ID already exists
    /// * Invalid data in the `meta_asset` that violates database constraints
    /// * Transaction failure during the insert operation
    ///
    pub fn create_asset(db: &mut Connection, meta_asset: MetaAsset) -> QueryResult<MetaAsset> {
        let asset = Self::from_meta_asset(meta_asset);
        match insert_into(assets_table)
            .values(&asset)
            .get_result::<Self>(db)
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
    /// Returns a `QueryResult<usize>` containing the number of assets successfully inserted.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Unique constraint violations if any asset IDs already exist
    /// * Invalid data in any `meta_asset` that violates database constraints
    /// * Transaction failure during the bulk insert operation
    /// * The operation is atomic - either all assets are inserted or none are
    ///
    pub fn create_asset_collection(
        conn: &mut Connection,
        meta_assets: &[MetaAsset],
    ) -> QueryResult<usize> {
        let items: Vec<Self> = meta_assets
            .iter()
            .map(|ma| Self::from_meta_asset(ma.clone()))
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
    /// Returns a `QueryResult<u64>` containing the total count of assets in the database.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Integer overflow when converting count from i64 to u64 (extremely unlikely)
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
    /// Returns a `QueryResult<bool>`:
    /// * `Ok(true)` if the asset exists
    /// * `Ok(false)` if the asset does not exist
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Type conversion errors when processing the result
    ///
    pub fn check_if_asset_id_exists(db: &mut Connection, asset_id: String) -> QueryResult<bool> {
        let exists = assets_table
            .find(asset_id)
            .first::<Self>(db)
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
    /// Returns a `QueryResult<Option<MetaAsset>>`:
    /// * `Ok(Some(meta_asset))` if the asset was found
    /// * `Ok(None)` if no asset exists with the given ID
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Data deserialization error when converting database record to `MetaAsset`
    ///
    pub fn read(db: &mut Connection, param_asset_id: String) -> QueryResult<Option<MetaAsset>> {
        let exists = Self::check_if_asset_id_exists(db, param_asset_id.clone())?;

        if !exists {
            Ok(None)
        } else {
            assets_table
                .filter(asset_code.eq(param_asset_id))
                .first::<Self>(db)
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
    /// Returns a `QueryResult<Vec<MetaAsset>>` containing all assets in the database.
    /// Returns an empty vector if no assets exist.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Data deserialization errors when converting database records to `MetaAsset`
    /// * Memory allocation errors when dealing with large result sets
    ///
    pub fn read_all(db: &mut Connection) -> QueryResult<Vec<MetaAsset>> {
        assets_table
            .load::<Self>(db)
            .map(|a| a.into_iter().map(|asset| asset.to_meta_asset()).collect())
    }

    /// Update an asset in the database by asset ID.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `asset_id` - The ID of the asset to update.
    /// * `meta_asset` - The new asset metadata to update with.
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<usize>` indicating the number of rows affected:
    /// * `Ok(1)` if the asset was successfully updated
    /// * `Ok(0)` if no asset exists with the given ID
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Constraint violations in the new `meta_asset` data
    /// * Transaction failure during the update operation
    /// * Concurrent modification conflicts
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
    /// Returns a `QueryResult<usize>` indicating the number of rows affected:
    /// * `Ok(1)` if the asset was successfully deleted
    /// * `Ok(0)` if no asset exists with the given ID
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Foreign key constraint violations if the asset is referenced by other tables
    /// * Transaction failure during the delete operation
    /// * Concurrent modification conflicts
    ///
    pub fn delete(db: &mut Connection, asset_id: String) -> QueryResult<usize> {
        diesel::delete(assets_table.find(asset_id)).execute(db)
    }
}
