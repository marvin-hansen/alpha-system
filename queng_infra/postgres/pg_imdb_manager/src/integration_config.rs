/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::PostgresIMDBManager;
use common_errors::PostgresDBError;
use common_ims::IntegrationConfig as ImsIntegrationConfig;
use pg_imdb::IntegrationConfig;

impl PostgresIMDBManager {
    /// Inserts a new IMS integration configuration into the database.
    ///
    /// # Arguments
    ///
    /// * `integration_config` - The IMS integration configuration to insert
    ///
    /// # Returns
    ///
    /// * `Result<ImsIntegrationConfig, PostgresDBError>` - The inserted configuration on success, or error on failure
    ///
    pub async fn insert_integration_config(
        &self,
        integration_config: ImsIntegrationConfig,
    ) -> Result<ImsIntegrationConfig, PostgresDBError> {
        self.dbg_print("insert_integration_config");
        let conn = &mut self.get_connection();

        match IntegrationConfig::create(conn, &integration_config) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    /// Inserts a collection of IMS integration configurations into the database.
    ///
    /// # Arguments
    ///
    /// * `integration_configs` - A slice of IMS integration configurations to insert
    ///
    /// # Returns
    ///
    /// * `Result<usize, PostgresDBError>` - Number of inserted configurations on success, or error on failure
    ///
    pub async fn insert_integration_config_collection(
        &self,
        integration_configs: &[ImsIntegrationConfig],
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("insert_integration_config_collection");
        let conn = &mut self.get_connection();

        match IntegrationConfig::insert_integration_config_collection(conn, integration_configs) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    /// Returns the total number of integration configurations in the database.
    ///
    /// # Returns
    ///
    /// A `Result` containing the count as `u64` if successful, or a `PostgresDBError` if the operation fails.
    ///
    pub async fn count_integration_configs(&self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_integration_configs");
        let conn = &mut self.get_connection();

        match IntegrationConfig::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
        }
    }

    /// Checks if an integration configuration exists in the database for the given ID.
    ///
    /// # Arguments
    ///
    /// * `integration_id` - The ID of the integration configuration to check
    ///
    /// # Returns
    ///
    /// A `Result` containing a boolean indicating if the configuration exists,
    /// or a `PostgresDBError` if the operation fails.
    ///
    pub async fn check_if_integration_config_exists(
        &self,
        integration_id: String,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_integration_config_exists");
        let conn = &mut self.get_connection();

        match IntegrationConfig::check_if_integration_config_exists(conn, integration_id) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        }
    }

    /// Checks if an integration configuration is online in the database.
    ///
    /// # Arguments
    ///
    /// * `integration_id` - The ID of the integration configuration to check
    ///
    /// # Returns
    ///
    /// A `Result` containing a boolean indicating if the integration is online,
    /// or a `PostgresDBError` if the check fails.
    ///
    pub async fn check_if_integration_config_online(
        &self,
        integration_id: String,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_integration_config_online");
        let conn = &mut self.get_connection();

        match IntegrationConfig::check_if_integration_config_online(conn, integration_id) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CheckFailed(e.to_string())),
        }
    }

    /// Retrieves an integration configuration from the database by its ID.
    ///
    /// # Arguments
    ///
    /// * `integration_id` - The ID of the integration configuration to retrieve
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<ImsIntegrationConfig>` with the retrieved configuration,
    /// or `None` if no configuration exists. Returns a `PostgresDBError` if the operation fails.
    ///
    pub async fn get_integrations_config(
        &self,
        integration_id: String,
    ) -> Result<Option<ImsIntegrationConfig>, PostgresDBError> {
        self.dbg_print("get_integrations_config");
        let conn = &mut self.get_connection();

        match IntegrationConfig::get_integration_config(conn, integration_id) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Retrieves all online integration configurations from the database.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `ImsIntegrationConfig` for all online integrations,
    /// or a `PostgresDBError` if the query fails.
    ///
    pub async fn get_all_online_integration_configs(
        &self,
    ) -> Result<Vec<ImsIntegrationConfig>, PostgresDBError> {
        self.dbg_print("get_all_online_integrations");
        let conn = &mut self.get_connection();

        match IntegrationConfig::get_all_online_integration_configs(conn) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Retrieves all offline integration configurations from the database.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<ImsIntegrationConfig>, PostgresDBError>` - A vector of offline integration configs if successful,
    ///   or a `PostgresDBError` if the query fails
    ///
    pub async fn get_all_offline_integration_configs(
        &self,
    ) -> Result<Vec<ImsIntegrationConfig>, PostgresDBError> {
        self.dbg_print("get_all_offline_integrations");
        let conn = &mut self.get_connection();

        match IntegrationConfig::get_all_offline_integration_configs(conn) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Retrieves all IMS integration configurations from the database.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<ImsIntegrationConfig>, PostgresDBError>` - List of integration configs on success,
    ///    or error on failure
    ///
    pub async fn get_integration_config(
        &self,
    ) -> Result<Vec<ImsIntegrationConfig>, PostgresDBError> {
        self.dbg_print("get_all_integrations");
        let conn = &mut self.get_connection();

        match IntegrationConfig::get_all_integration_configs(conn) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Retrieves all integration configurations for a specific exchange.
    ///
    /// # Arguments
    ///
    /// * `exchange_id` - The ID of the exchange to filter configurations by
    ///
    /// # Returns
    ///
    /// * `Result<Vec<ImsIntegrationConfig>, PostgresDBError>` - List of integration configs
    ///    for the exchange on success, or error on failure
    ///
    pub async fn get_all_integration_configs_by_exchange(
        &self,
        exchange_id: i32,
    ) -> Result<Vec<ImsIntegrationConfig>, PostgresDBError> {
        self.dbg_print("get_all_integrations_by_exchange_id");
        let conn = &mut self.get_connection();

        match IntegrationConfig::get_all_integration_configs_by_exchange(conn, exchange_id) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Sets an integration configuration to online state in the database.
    ///
    /// # Arguments
    /// * `integration_id` - The unique identifier of the integration to set online
    ///
    /// # Returns
    ///
    /// * `Result<(), PostgresDBError>` - Ok if successful, `PostgresDBError` if update fails
    pub async fn set_integration_online(
        &self,
        integration_id: String,
    ) -> Result<(), PostgresDBError> {
        self.dbg_print("set_integration_online");
        let conn = &mut self.get_connection();

        match IntegrationConfig::set_integration_config_online(conn, integration_id) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::UpdateFailed(e.to_string())),
        }
    }

    /// Sets an integration configuration to offline state in the database.
    ///
    /// # Arguments
    /// * `integration_id` - The unique identifier of the integration to set offline
    ///
    /// # Returns
    /// * `Result<(), PostgresDBError>` - Ok if successful, `PostgresDBError` if update fails
    ///
    pub async fn set_integration_offline(
        &self,
        integration_id: String,
    ) -> Result<(), PostgresDBError> {
        self.dbg_print("set_integration_online");
        let conn = &mut self.get_connection();

        match IntegrationConfig::set_integration_config_offline(conn, integration_id) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::UpdateFailed(e.to_string())),
        }
    }

    /// Updates an integration configuration in the database.
    ///
    /// # Arguments
    /// * `integration_config` - The IMS integration configuration to update
    ///
    /// # Returns
    ///
    /// * `Result<usize, PostgresDBError>` - Number of rows updated on success, or error on failure
    ///
    pub async fn update_integration_config(
        &self,
        integration_config: ImsIntegrationConfig,
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("update_integration_config");
        let conn = &mut self.get_connection();

        match IntegrationConfig::update_integration_config(conn, integration_config) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::UpdateFailed(e.to_string())),
        }
    }

    /// Deletes an integration configuration from the database.
    ///
    /// # Arguments
    /// * `integration_id` - The unique identifier of the integration configuration to delete
    ///
    /// # Returns
    /// * `Result<usize, PostgresDBError>` - Number of rows affected on success, or error on failure
    pub async fn delete_integration_config(
        &self,
        integration_id: String,
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("delete_integration_config");
        let conn = &mut self.get_connection();

        match IntegrationConfig::delete_integration_config(conn, integration_id) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::DeleteFailed(e.to_string())),
        }
    }
}
