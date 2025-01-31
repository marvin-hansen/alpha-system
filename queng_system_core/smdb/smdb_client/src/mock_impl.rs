/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::{SMDBCMockClient, SMDBError, SmdbClientTrait};
use async_trait::async_trait;
use common_config::ServiceID;

#[async_trait]
impl SmdbClientTrait for SMDBCMockClient {
    async fn check_if_service_id_exists(&self, _id: ServiceID) -> Result<bool, SMDBError> {
        Ok(true)
    }

    async fn check_if_services_exists(&self, _services: Vec<ServiceID>) -> Result<bool, SMDBError> {
        Ok(true)
    }

    async fn check_if_service_id_online(&self, _id: ServiceID) -> Result<bool, SMDBError> {
        Ok(true)
    }

    async fn check_if_services_online(&self, _services: Vec<ServiceID>) -> Result<bool, SMDBError> {
        Ok(true)
    }

    async fn set_service_online(&self, _id: ServiceID) -> Result<bool, SMDBError> {
        Ok(true)
    }

    async fn set_service_offline(&self, _id: ServiceID) -> Result<bool, SMDBError> {
        Ok(true)
    }
}
