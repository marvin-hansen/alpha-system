/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::imdb_error::IMDBClientError;
use crate::{IMDBCMockClient, ImdbClientTrait};
use async_trait::async_trait;
use common_ims::{ExchangeID, IntegrationConfig};

#[async_trait]
impl ImdbClientTrait for IMDBCMockClient {
    async fn count_integrations(&self) -> Result<u64, IMDBClientError> {
        Ok(1)
    }

    async fn check_if_integration_exists(
        &self,
        _integration_id: String,
    ) -> Result<bool, IMDBClientError> {
        Ok(true)
    }

    async fn check_if_integration_online(
        &self,
        _integration_id: String,
    ) -> Result<bool, IMDBClientError> {
        Ok(true)
    }

    async fn get_integration(
        &self,
        _integration_id: String,
    ) -> Result<Option<IntegrationConfig>, IMDBClientError> {
        Err(IMDBClientError::from("Not implemented in Mock client"))
    }

    async fn get_all_integrations(&self) -> Result<Vec<IntegrationConfig>, IMDBClientError> {
        Err(IMDBClientError::from("Not implemented in Mock client"))
    }

    async fn get_all_integrations_by_exchange(
        &self,
        _exchange_id: ExchangeID,
    ) -> Result<Vec<IntegrationConfig>, IMDBClientError> {
        Err(IMDBClientError::from("Not implemented in Mock client"))
    }

    async fn get_all_online_integrations(&self) -> Result<Vec<IntegrationConfig>, IMDBClientError> {
        Err(IMDBClientError::from("Not implemented in Mock client"))
    }

    async fn get_all_offline_integrations(
        &self,
    ) -> Result<Vec<IntegrationConfig>, IMDBClientError> {
        Err(IMDBClientError::from("Not implemented in Mock client"))
    }

    async fn set_integration_online(&self, _integration_id: String) -> Result<(), IMDBClientError> {
        Ok(())
    }

    async fn set_integration_offline(
        &self,
        _integration_id: String,
    ) -> Result<(), IMDBClientError> {
        Ok(())
    }
}
