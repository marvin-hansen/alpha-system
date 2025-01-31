/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::imdb_client_trait::ImdbClientTrait;
use crate::imdb_error::IMDBClientError;
use crate::IMDBClient;
use async_trait::async_trait;
use common_ims::{ExchangeID, IntegrationConfig};
use proto_imdb::proto::ProtoIntegrationConfig;
use proto_imdb_utils::*;

#[async_trait]
impl ImdbClientTrait for IMDBClient {
    /// Counts the total number of integrations in the database
    ///
    /// # Errors
    /// * `IMDBClientError` - If the operation fails
    ///
    /// # Returns
    /// * `Result<u64, IMDBClientError>` - The total number of integrations, or an error if the operation fails
    ///
    async fn count_integrations(&self) -> Result<u64, IMDBClientError> {
        let mut client = self.client.clone();
        let request = get_count_integration_request();

        match client.count_integration_configs(request).await {
            Ok(res) => Ok(res.into_inner().count),
            Err(e) => Err(IMDBClientError(e.to_string())),
        }
    }
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
    ) -> Result<bool, IMDBClientError> {
        let mut client = self.client.clone();
        let request = get_check_if_integration_config_exists_request(integration_id);
        match client.check_if_integration_config_exists(request).await {
            Ok(res) => Ok(res.into_inner().exists),
            Err(e) => Err(IMDBClientError(e.to_string())),
        }
    }
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
    ) -> Result<bool, IMDBClientError> {
        let mut client = self.client.clone();
        let request = get_check_if_integration_config_online_request(integration_id);
        match client.check_if_integration_config_online(request).await {
            Ok(res) => Ok(res.into_inner().online),
            Err(e) => Err(IMDBClientError(e.to_string())),
        }
    }
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
    ) -> Result<Option<IntegrationConfig>, IMDBClientError> {
        let mut client = self.client.clone();
        let request = get_integration_request(&integration_id);
        match client.get_integration_config(request).await {
            Ok(res) => {
                let proto_integration = res.into_inner().integration;

                // Convert Option<ProtoIntegrationConfig> to Option<IntegrationConfig>
                let integration = proto_integration.map(integration_config_from_proto);

                Ok(integration)
            }
            Err(e) => Err(IMDBClientError(e.to_string())),
        }
    }
    /// Gets all integrations from the database
    ///
    /// # Errors
    /// * `IMDBClientError` - If the operation fails
    ///
    /// # Returns
    /// * `Result<Vec<IntegrationConfig>, IMDBClientError>` - `Ok(Vec<IntegrationConfig>)` if the operation was successful, or an error if it failed
    ///
    async fn get_all_integrations(&self) -> Result<Vec<IntegrationConfig>, IMDBClientError> {
        let mut client = self.client.clone();
        let request = get_all_integrations_request();
        match client.get_all_integration_configs(request).await {
            Ok(res) => {
                // Extract the Vec<ProtoIntegrationConfig>
                let integrations = res.into_inner().integrations;

                // Convert Vec<ProtoIntegrationConfig> to Vec<IntegrationConfig>
                let integrations = integrations
                    .iter()
                    .map(|proto: &ProtoIntegrationConfig| {
                        integration_config_from_proto(proto.clone())
                    })
                    .collect();

                Ok(integrations)
            }
            Err(e) => Err(IMDBClientError(e.to_string())),
        }
    }
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
    ) -> Result<Vec<IntegrationConfig>, IMDBClientError> {
        let mut client = self.client.clone();
        let request = get_all_integrations_by_exchange_request(exchange_id);
        match client
            .get_all_integration_configs_by_exchange(request)
            .await
        {
            Ok(res) => {
                // Extract the Vec<ProtoIntegrationConfig>
                let integrations = res.into_inner().integrations;

                // Convert Vec<ProtoIntegrationConfig> to Vec<IntegrationConfig>
                let integrations = integrations
                    .iter()
                    .map(|proto: &ProtoIntegrationConfig| {
                        integration_config_from_proto(proto.clone())
                    })
                    .collect();

                Ok(integrations)
            }
            Err(e) => Err(IMDBClientError(e.to_string())),
        }
    }
    /// Gets all online integrations from the database
    ///
    /// # Errors
    /// * `IMDBClientError` - If the operation fails
    ///
    /// # Returns
    /// * `Result<Vec<IntegrationConfig>, IMDBClientError>` - `Ok(Vec<IntegrationConfig>)` if the operation was successful, or an error if it failed
    ///
    async fn get_all_online_integrations(&self) -> Result<Vec<IntegrationConfig>, IMDBClientError> {
        let mut client = self.client.clone();
        let request = get_all_online_integrations_request();
        match client.get_all_online_integration_configs(request).await {
            Ok(res) => {
                // Extract the Vec<ProtoIntegrationConfig>
                let integrations = res.into_inner().integrations;

                // Convert Vec<ProtoIntegrationConfig> to Vec<IntegrationConfig>
                let integrations = integrations
                    .iter()
                    .map(|proto: &ProtoIntegrationConfig| {
                        integration_config_from_proto(proto.clone())
                    })
                    .collect();

                Ok(integrations)
            }
            Err(e) => Err(IMDBClientError(e.to_string())),
        }
    }
    /// Gets all offline integrations from the database
    ///
    /// # Errors
    /// * `IMDBClientError` - If the operation fails
    ///
    /// # Returns
    /// * `Result<Vec<IntegrationConfig>, IMDBClientError>` - `Ok(Vec<IntegrationConfig>)` if the operation was successful, or an error if it failed
    ///
    async fn get_all_offline_integrations(
        &self,
    ) -> Result<Vec<IntegrationConfig>, IMDBClientError> {
        let mut client = self.client.clone();
        let request = get_all_offline_integrations_request();

        match client.get_all_offline_integration_configs(request).await {
            Ok(res) => {
                // Extract the Vec<ProtoIntegrationConfig>
                let integrations = res.into_inner().integrations;

                // Convert Vec<ProtoIntegrationConfig> to Vec<IntegrationConfig>
                let integrations = integrations
                    .iter()
                    .map(|proto: &ProtoIntegrationConfig| {
                        integration_config_from_proto(proto.clone())
                    })
                    .collect();

                Ok(integrations)
            }
            Err(e) => Err(IMDBClientError(e.to_string())),
        }
    }
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
    async fn set_integration_online(&self, integration_id: String) -> Result<(), IMDBClientError> {
        let mut client = self.client.clone();
        let request = get_set_integration_online_request(&integration_id);
        match client.set_integration_online(request).await {
            Ok(res) => {
                let res = res.into_inner();
                if res.ok {
                    Ok(())
                } else {
                    Err(IMDBClientError(res.error.unwrap()))
                }
            }
            Err(e) => Err(IMDBClientError(e.to_string())),
        }
    }
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
    async fn set_integration_offline(&self, integration_id: String) -> Result<(), IMDBClientError> {
        let mut client = self.client.clone();
        let request = get_set_integration_offline_request(&integration_id);
        match client.set_integration_offline(request).await {
            Ok(res) => {
                let res = res.into_inner();
                if res.ok {
                    Ok(())
                } else {
                    Err(IMDBClientError(res.error.unwrap()))
                }
            }
            Err(e) => Err(IMDBClientError(e.to_string())),
        }
    }
}
