use crate::model::integration_config::{
    CreateIntegrationConfig, IntegrationConfig, UpdateIntegrationConfig,
};
use crate::schema::imdb::integration_config::dsl::*;
use crate::Connection;
use common_ims::prelude::IntegrationConfig as CommonIntegrationConfig;
use diesel::dsl::{delete, insert_into};
use diesel::prelude::*;
use diesel::{QueryResult, RunQueryDsl};

impl IntegrationConfig {
    /// Creates a new integration configuration in the database with the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    /// * `svc` - a reference to a `CommonIntegrationConfig` containing the configuration for the new integration
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing the configuration of the newly created integration,
    /// or an error if the operation fails.
    ///
    pub fn create(
        db: &mut Connection,
        svc: &CommonIntegrationConfig,
    ) -> QueryResult<CommonIntegrationConfig> {
        let item = CreateIntegrationConfig::from_common_integration_config(svc.clone());
        insert_into(crate::schema::imdb::integration_config::table)
            .values(item)
            .get_result::<IntegrationConfig>(db)
            .map(|s| s.to_common_integration_config())
    }

    /// Inserts a collection of integration configurations into the database.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    /// * `configs` - a vector of `CommonIntegrationConfig` containing the configurations to be inserted
    ///
    /// # Returns
    ///
    /// A `QueryResult<bool>` indicating whether the operation was successful.
    ///
    /// The function returns `Ok(true)` if the insertion is successful, otherwise it returns an `Err`
    /// containing the error that occurred during the operation.
    pub fn insert_integration_config_collection(
        db: &mut Connection,
        configs: Vec<CommonIntegrationConfig>,
    ) -> QueryResult<bool> {
        let items: Vec<CreateIntegrationConfig> = configs
            .into_iter()
            .map(CreateIntegrationConfig::from_common_integration_config)
            .collect();

        match insert_into(crate::schema::imdb::integration_config::table)
            .values(&items)
            .execute(db)
        {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    /// Retrieves the number of integration configurations in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing the count of integration configurations as `u64` if successful, or an error.
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
    /// * `db` - a mutable reference to a postgres database connection
    /// * `config_id` - the ID of the integration configuration to check
    ///
    /// # Returns
    ///
    /// A `QueryResult<bool>` indicating whether the integration configuration exists or not.
    /// If the operation fails, returns an error.
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
    /// * `db` - a mutable reference to a postgres database connection
    /// * `config_id` - the ID of the integration configuration to check
    ///
    /// # Returns
    ///
    /// A `QueryResult<bool>` indicating whether the integration configuration is online or not.
    /// If the operation fails, returns an error.
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
    /// * `db` - a mutable reference to a postgres database connection
    /// * `config_id` - the ID of the integration configuration to retrieve
    ///
    /// # Returns
    ///
    /// A `QueryResult<Option<CommonIntegrationConfig>>` containing the retrieved integration configuration,
    /// or `None` if no configuration with the given ID exists. If the operation fails, returns an error.
    ///
    pub fn get_integration_config(
        db: &mut Connection,
        config_id: String,
    ) -> QueryResult<Option<CommonIntegrationConfig>> {
        integration_config
            .filter(integration_id.eq(config_id))
            .first::<IntegrationConfig>(db)
            .optional()
            .map(|opt| opt.map(|config| config.to_common_integration_config()))
    }

    /// Retrieves all integration configurations from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// A `QueryResult<Vec<CommonIntegrationConfig>>` containing all integration configurations in the database,
    /// or an error if the operation fails.
    pub fn get_all_integration_configs(
        db: &mut Connection,
    ) -> QueryResult<Vec<CommonIntegrationConfig>> {
        integration_config
            .load::<IntegrationConfig>(db)
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
    /// * `db` - a mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// A `QueryResult<Vec<CommonIntegrationConfig>>` containing all online integration configurations in the database,
    /// or an error if the operation fails.
    pub fn get_all_online_integration_configs(
        db: &mut Connection,
    ) -> QueryResult<Vec<CommonIntegrationConfig>> {
        integration_config
            .filter(online.eq(true))
            .load::<IntegrationConfig>(db)
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
    /// * `db` - a mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// A `QueryResult<Vec<CommonIntegrationConfig>>` containing all offline integration configurations in the database,
    /// or an error if the operation fails.
    pub fn get_all_offline_integration_configs(
        db: &mut Connection,
    ) -> QueryResult<Vec<CommonIntegrationConfig>> {
        integration_config
            .filter(online.eq(false))
            .load::<IntegrationConfig>(db)
            .map(|configs| {
                configs
                    .into_iter()
                    .map(|c| c.to_common_integration_config())
                    .collect()
            })
    }

    /// Updates an integration configuration in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    /// * `config` - the `CommonIntegrationConfig` to update
    ///
    /// # Returns
    ///
    /// A `QueryResult<CommonIntegrationConfig>` containing the updated integration configuration,
    /// or an error if the operation fails.
    pub fn update_integration_config(
        db: &mut Connection,
        config: CommonIntegrationConfig,
    ) -> QueryResult<CommonIntegrationConfig> {
        let update_config = UpdateIntegrationConfig::from_common_integration_config(config);
        diesel::update(
            integration_config.filter(integration_id.eq(&update_config.integration_id.clone())),
        )
        .set(update_config)
        .get_result::<IntegrationConfig>(db)
        .map(|config| config.to_common_integration_config())
    }

    /// Deletes an integration configuration from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    /// * `config_id` - the ID of the integration configuration to delete
    ///
    /// # Returns
    ///
    /// A `QueryResult<usize>` containing the number of rows affected by the delete operation.
    /// If the operation fails, returns an error.
    pub fn delete_integration_config(db: &mut Connection, config_id: String) -> QueryResult<usize> {
        delete(integration_config.filter(integration_id.eq(config_id))).execute(db)
    }
}
