use crate::imdb_error::IMDBClientError;
use async_trait::async_trait;
use common_ims::{ExchangeID, IntegrationConfig};
use enum_dispatch::enum_dispatch;

#[async_trait]
#[enum_dispatch(IMDBClientSelector)]
pub trait ImdbClientTrait {
    /// Counts the total number of integrations in the database
    ///
    /// # Errors
    /// * `IMDBClientError` - If the operation fails
    ///
    /// # Returns
    /// * `Result<u64, IMDBClientError>` - The total number of integrations, or an error if the operation fails
    ///
    async fn count_integrations(&self) -> Result<u64, IMDBClientError>;
    /// Checks if an integration with the given ID exists in the database
    ///
    /// # Arguments
    /// * `integration_id` - The ID of the integration to check
    ///
    /// # Errors
    /// * `IMDBClientError` - If the operation fails
    ///
    /// # Returns
    /// * `Result<bool, IMDBClientError>` - `true` if the integration exists, `false` if the integration does not exist, or an error if the operation fails
    ///
    async fn check_if_integration_exists(
        &self,
        integration_id: String,
    ) -> Result<bool, IMDBClientError>;
    /// Checks if an integration with the given ID is online in the database
    ///
    /// # Arguments
    /// * `integration_id` - The ID of the integration to check
    ///
    /// # Errors
    /// * `IMDBClientError` - If the operation fails
    ///
    /// # Returns
    /// * `Result<bool, IMDBClientError>` - `true` if the integration is online, `false` if the integration is offline, or an error if the operation fails
    ///
    async fn check_if_integration_online(
        &self,
        integration_id: String,
    ) -> Result<bool, IMDBClientError>;
    /// Retrieves an integration from the database by its ID
    ///
    /// # Arguments
    /// * `integration_id` - The ID of the integration to retrieve
    ///
    /// # Errors
    /// * `IMDBClientError` - If the operation fails
    ///
    /// # Returns
    /// * `Result<Option<IntegrationConfig>, IMDBClientError>` - `Ok(Some(IntegrationConfig))` if the integration was found, `Ok(None)` if the integration was not found, or an error if the operation fails
    ///
    async fn get_integration(
        &self,
        integration_id: String,
    ) -> Result<Option<IntegrationConfig>, IMDBClientError>;
    /// Gets all integrations from the database
    ///
    /// # Errors
    /// * `IMDBClientError` - If the operation fails
    ///
    /// # Returns
    /// * `Result<Vec<IntegrationConfig>, IMDBClientError>` - `Ok(Vec<IntegrationConfig>)` if the operation was successful, or an error if it failed
    ///
    async fn get_all_integrations(&self) -> Result<Vec<IntegrationConfig>, IMDBClientError>;
    /// Gets all integrations for a specific exchange from the database
    ///
    /// # Arguments
    /// * `exchange_id` - The ID of the exchange to retrieve integrations for
    ///
    /// # Errors
    /// * `IMDBClientError` - If the operation fails
    ///
    /// # Returns
    /// * `Result<Vec<IntegrationConfig>, IMDBClientError>` - `Ok(Vec<IntegrationConfig>)` if the operation was successful, or an error if it failed
    ///
    async fn get_all_integrations_by_exchange(
        &self,
        exchange_id: ExchangeID,
    ) -> Result<Vec<IntegrationConfig>, IMDBClientError>;
    /// Gets all online integrations from the database
    ///
    /// # Errors
    /// * `IMDBClientError` - If the operation fails
    ///
    /// # Returns
    /// * `Result<Vec<IntegrationConfig>, IMDBClientError>` - `Ok(Vec<IntegrationConfig>)` if the operation was successful, or an error if it failed
    ///
    async fn get_all_online_integrations(&self) -> Result<Vec<IntegrationConfig>, IMDBClientError>;
    /// Gets all offline integrations from the database
    ///
    /// # Errors
    /// * `IMDBClientError` - If the operation fails
    ///
    /// # Returns
    /// * `Result<Vec<IntegrationConfig>, IMDBClientError>` - `Ok(Vec<IntegrationConfig>)` if the operation was successful, or an error if it failed
    ///
    async fn get_all_offline_integrations(&self)
        -> Result<Vec<IntegrationConfig>, IMDBClientError>;
    /// Sets the integration with the given ID to online
    ///
    /// # Arguments
    /// * `integration_id` - The ID of the integration to set online
    ///
    /// # Errors
    /// * `IMDBClientError` - If the operation fails
    ///
    /// # Returns
    /// * `Result<(), IMDBClientError>` - `Ok(())` if the operation was successful, or an error if it failed
    ///
    async fn set_integration_online(&self, integration_id: String) -> Result<(), IMDBClientError>;
    /// Sets the integration with the given ID to offline
    ///
    /// # Arguments
    /// * `integration_id` - The ID of the integration to set offline
    ///
    /// # Errors
    /// * `IMDBClientError` - If the operation fails
    ///
    /// # Returns
    /// * `Result<(), IMDBClientError>` - `Ok(())` if the operation was successful, or an error if it failed
    ///
    async fn set_integration_offline(&self, integration_id: String) -> Result<(), IMDBClientError>;
}
