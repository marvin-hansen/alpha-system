/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::model::integration_config::{
    CreateIntegrationConfig, IntegrationConfig, UpdateIntegrationConfig,
};
use crate::schema::imdb::integration_config::dsl::{
    exchange_id, integration_config, integration_id, online,
};
use crate::Connection;
use common_ims::IntegrationConfig as CommonIntegrationConfig;
use diesel::dsl::{delete, insert_into};
use diesel::prelude::*;
use diesel::{QueryResult, RunQueryDsl};

impl IntegrationConfig {
    /// Creates a new integration configuration in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `config` - A reference to a `CommonIntegrationConfig` containing the configuration for the new integration
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<CommonIntegrationConfig>`:
    /// * `Ok(config)` - The newly created integration configuration
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Unique constraint violations (e.g., duplicate `integration_id`)
    /// * Invalid data in config that violates database constraints
    /// * Transaction failure during the insert operation
    /// * Data conversion errors between `CommonIntegrationConfig` and database types
    ///
    /// # Implementation Notes
    ///
    /// This function:
    /// 1. Converts the `CommonIntegrationConfig` to a database-specific `CreateIntegrationConfig`
    /// 2. Performs the insert operation with automatic returning of the created record
    /// 3. Converts the result back to `CommonIntegrationConfig` format
    ///
    /// # Performance Considerations
    ///
    /// * Uses a single database roundtrip for insert and return
    /// * Requires validation of unique constraints before insertion
    ///
    pub fn create(
        db: &mut Connection,
        config: &CommonIntegrationConfig,
    ) -> QueryResult<CommonIntegrationConfig> {
        let item = CreateIntegrationConfig::from_common_integration_config(config);
        insert_into(crate::schema::imdb::integration_config::table)
            .values(item)
            .get_result::<Self>(db)
            .map(|s| s.to_common_integration_config())
    }

    /// Inserts a collection of integration configurations into the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `configs` - A slice of `CommonIntegrationConfig` containing the configurations to be inserted
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<usize>`:
    /// * `Ok(n)` - Number of configurations successfully inserted
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Unique constraint violations (e.g., duplicate `integration_ids`)
    /// * Invalid data in any config that violates database constraints
    /// * Transaction failure during the bulk insert operation
    /// * Data conversion errors between `CommonIntegrationConfig` and database types
    /// * The operation is atomic - either all configs are inserted or none are
    ///
    pub fn insert_integration_config_collection(
        db: &mut Connection,
        configs: &[CommonIntegrationConfig],
    ) -> QueryResult<usize> {
        let items: Vec<CreateIntegrationConfig> = configs
            .iter()
            .map(
                |common_integration_config: &common_ims::IntegrationConfig| {
                    CreateIntegrationConfig::from_common_integration_config(
                        common_integration_config,
                    )
                },
            )
            .collect();

        match insert_into(crate::schema::imdb::integration_config::table)
            .values(&items)
            .execute(db)
        {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    /// Retrieves the number of integration configurations in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<u64>`:
    /// * `Ok(count)` - The total number of integration configurations
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Integer overflow when converting count from i64 to u64 (extremely unlikely)
    ///
    pub fn count(db: &mut Connection) -> QueryResult<u64> {
        integration_config
            .count()
            .get_result::<i64>(db)
            .map(|c| c as u64)
    }

    /// Checks if an integration configuration with the given ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `config_id` - The ID of the integration configuration to check
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<bool>`:
    /// * `Ok(true)` - The integration configuration exists
    /// * `Ok(false)` - No integration configuration exists with the given ID
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Type conversion errors when processing the result
    ///
    pub fn check_if_integration_config_exists(
        db: &mut Connection,
        config_id: String,
    ) -> QueryResult<bool> {
        integration_config
            .filter(integration_id.eq(config_id))
            .select(integration_id)
            .first::<String>(db)
            .optional()
            .map(|result| result.is_some())
    }

    /// Checks if an integration configuration with the given ID is online.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `config_id` - The ID of the integration configuration to check
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<bool>`:
    /// * `Ok(true)` - The integration configuration is online
    /// * `Ok(false)` - The integration configuration is offline or does not exist
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Type conversion errors when processing the result
    ///
    pub fn check_if_integration_config_online(
        db: &mut Connection,
        config_id: String,
    ) -> QueryResult<bool> {
        integration_config
            .filter(integration_id.eq(config_id))
            .select(online)
            .first(db)
    }

    /// Retrieves an integration configuration from the database by its ID.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `config_id` - The ID of the integration configuration to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<Option<CommonIntegrationConfig>>`:
    /// * `Ok(Some(config))` - The integration configuration was found
    /// * `Ok(None)` - No integration configuration exists with the given ID
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Data deserialization errors when converting database record to `CommonIntegrationConfig`
    ///
    pub fn get_integration_config(
        db: &mut Connection,
        config_id: String,
    ) -> QueryResult<Option<CommonIntegrationConfig>> {
        integration_config
            .filter(integration_id.eq(config_id))
            .first::<Self>(db)
            .optional()
            .map(|opt| opt.map(|config| config.to_common_integration_config()))
    }

    /// Retrieves all integration configurations from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<Vec<CommonIntegrationConfig>>`:
    /// * `Ok(vec)` - A vector containing all integration configurations
    /// * Returns an empty vector if no configurations exist
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Data deserialization errors when converting database records
    /// * Memory allocation errors when dealing with large result sets
    ///
    /// # Implementation Notes
    ///
    /// This function:
    /// 1. Queries all records from the `integration_config` table
    /// 2. Converts each record to `CommonIntegrationConfig` format
    /// 3. Collects results into a vector
    ///
    /// # Performance Considerations
    ///
    /// * For large datasets, consider using pagination or limiting the result set
    /// * Memory usage scales linearly with the number of configurations
    /// * Consider using `get_all_integration_configs_by_exchange` for filtered results
    ///
    pub fn get_all_integration_configs(
        db: &mut Connection,
    ) -> QueryResult<Vec<CommonIntegrationConfig>> {
        integration_config.load::<Self>(db).map(|configs| {
            configs
                .into_iter()
                .map(|c| c.to_common_integration_config())
                .collect()
        })
    }

    /// Retrieves all integration configurations for a specific exchange from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `param_exchange_id` - The ID of the exchange to filter configurations by
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<Vec<CommonIntegrationConfig>>`:
    /// * `Ok(vec)` - A vector containing all configurations for the specified exchange
    /// * Returns an empty vector if no configurations exist for the exchange
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Database connection error
    /// * Query execution failure
    /// * Data deserialization errors when converting database records
    /// * Invalid `exchange_id` format or type
    ///
    /// # Implementation Notes
    ///
    /// This function:
    /// 1. Filters `integration_config` table by the specified `exchange_id`
    /// 2. Converts matching records to `CommonIntegrationConfig` format
    /// 3. Returns results as a vector
    ///
    /// # Performance Considerations
    ///
    /// * Uses an index on `exchange_id` for efficient filtering
    /// * Memory usage scales with the number of configurations per exchange
    /// * More efficient than `get_all_integration_configs` when filtering by exchange
    ///
    pub fn get_all_integration_configs_by_exchange(
        db: &mut Connection,
        param_exchange_id: i32,
    ) -> QueryResult<Vec<CommonIntegrationConfig>> {
        integration_config
            .filter(exchange_id.eq(param_exchange_id))
            .load::<Self>(db)
            .map(|configs| {
                configs
                    .into_iter()
                    .map(|c| c.to_common_integration_config())
                    .collect()
            })
    }

    /// Retrieves all online integration configurations from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<Vec<CommonIntegrationConfig>>`:
    /// * `Ok(vec)` - A vector containing all online integration configurations
    /// * Returns an empty vector if no online configurations exist
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Data deserialization errors when converting database records
    /// * Memory allocation errors when dealing with large result sets
    ///
    /// # Implementation Notes
    ///
    /// This function:
    /// 1. Filters `integration_config` table where online = true
    /// 2. Converts matching records to `CommonIntegrationConfig` format
    /// 3. Returns results as a vector
    ///
    /// # Performance Considerations
    ///
    /// * Uses an index on the online field for efficient filtering
    /// * More efficient than filtering in application code
    /// * Consider implementing pagination for large result sets
    ///
    pub fn get_all_online_integration_configs(
        db: &mut Connection,
    ) -> QueryResult<Vec<CommonIntegrationConfig>> {
        integration_config
            .filter(online.eq(true))
            .load::<Self>(db)
            .map(|configs| {
                configs
                    .into_iter()
                    .map(|c| c.to_common_integration_config())
                    .collect()
            })
    }

    /// Retrieves all offline integration configurations from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<Vec<CommonIntegrationConfig>>`:
    /// * `Ok(vec)` - A vector containing all offline integration configurations
    /// * Returns an empty vector if no offline configurations exist
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Data deserialization errors when converting database records
    /// * Memory allocation errors when dealing with large result sets
    ///
    /// # Implementation Notes
    ///
    /// This function:
    /// 1. Filters `integration_config` table where online = false
    /// 2. Converts matching records to `CommonIntegrationConfig` format
    /// 3. Returns results as a vector
    ///
    /// # Performance Considerations
    ///
    /// * Uses an index on the online field for efficient filtering
    /// * More efficient than filtering in application code
    /// * Consider implementing pagination for large result sets
    ///
    pub fn get_all_offline_integration_configs(
        db: &mut Connection,
    ) -> QueryResult<Vec<CommonIntegrationConfig>> {
        integration_config
            .filter(online.eq(false))
            .load::<Self>(db)
            .map(|configs| {
                configs
                    .into_iter()
                    .map(|c| c.to_common_integration_config())
                    .collect()
            })
    }

    pub fn set_integration_config_online(
        db: &mut Connection,
        param_integration_id: String,
    ) -> QueryResult<()> {
        Self::set_online(db, param_integration_id, true)
    }

    /// Sets an integration configuration's online status to true.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `param_integration_id` - The ID of the integration configuration to update
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<()>`:
    /// * `Ok(())` - The online status was successfully updated
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * The integration configuration does not exist
    /// * Query execution failure
    /// * Concurrent modification conflicts
    ///
    /// # Implementation Notes
    ///
    /// This function:
    /// 1. Uses the generic `set_online` function with online=true
    /// 2. Updates only the online status field
    /// 3. Performs an atomic update operation
    ///
    pub fn set_integration_config_offline(
        db: &mut Connection,
        param_integration_id: String,
    ) -> QueryResult<()> {
        Self::set_online(db, param_integration_id, false)
    }

    /// Sets an integration configuration's online status to false.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `param_integration_id` - The ID of the integration configuration to update
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<()>`:
    /// * `Ok(())` - The online status was successfully updated
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * The integration configuration does not exist
    /// * Query execution failure
    /// * Concurrent modification conflicts
    ///
    /// # Implementation Notes
    ///
    /// This function:
    /// 1. Uses the generic `set_online` function with online=false
    /// 2. Updates only the online status field
    /// 3. Performs an atomic update operation
    ///
    fn set_online(
        db: &mut Connection,
        param_integration_id: String,
        param_online: bool,
    ) -> QueryResult<()> {
        match diesel::update(integration_config.filter(integration_id.eq(param_integration_id)))
            .set(online.eq(param_online))
            .execute(db)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Updates an integration configuration in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `config` - The updated `CommonIntegrationConfig` configuration
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<usize>`:
    /// * `Ok(1)` - The configuration was successfully updated
    /// * `Ok(0)` - No configuration exists with the given ID
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Constraint violations in the updated configuration
    /// * Data conversion errors between `CommonIntegrationConfig` and database types
    /// * Concurrent modification conflicts
    ///
    pub fn update_integration_config(
        db: &mut Connection,
        config: CommonIntegrationConfig,
    ) -> QueryResult<usize> {
        let update_config = UpdateIntegrationConfig::from_common_integration_config(config);
        diesel::update(
            integration_config.filter(integration_id.eq(&update_config.integration_id.clone())),
        )
        .set(update_config)
        .execute(db)
    }

    /// Deletes an integration configuration from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `config_id` - The ID of the integration configuration to delete
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<usize>`:
    /// * `Ok(1)` - The configuration was successfully deleted
    /// * `Ok(0)` - No configuration exists with the given ID
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection error
    /// * Query execution failure
    /// * Foreign key constraint violations if the configuration is referenced by other tables
    /// * Transaction failure during the delete operation
    /// * Concurrent modification conflicts
    ///
    pub fn delete_integration_config(db: &mut Connection, config_id: String) -> QueryResult<usize> {
        delete(integration_config.filter(integration_id.eq(config_id))).execute(db)
    }
}
