/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::SMDBError;
use async_trait::async_trait;
use common_config::ServiceID;
use enum_dispatch::enum_dispatch;

#[async_trait]
#[enum_dispatch(SMDBClientSelector)]
pub trait SmdbClientTrait {
    /// Checks if a service with the given ID exists in the SMDB.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to check.
    ///
    /// # Returns
    ///
    /// A `Result` that contains a boolean indicating whether the service exists or an
    /// `SMDBError` on failure.
    ///
    async fn check_if_service_id_exists(&self, id: ServiceID) -> Result<bool, SMDBError>;
    /// Checks if multiple services with the given IDs exist in the SMDB.
    ///
    /// # Arguments
    ///
    /// * `services` - The IDs of the services to check.
    ///
    /// # Returns
    ///
    /// A `Result` that contains a boolean indicating whether all the services exist or an
    /// `SMDBError` on failure.
    ///
    async fn check_if_services_exists(&self, services: Vec<ServiceID>) -> Result<bool, SMDBError>;
    async fn check_if_service_id_online(&self, id: ServiceID) -> Result<bool, SMDBError>;
    /// Checks if multiple services with the given IDs are online in the SMDB.
    ///
    /// # Arguments
    ///
    /// * `services` - The IDs of the services to check.
    ///
    /// # Returns
    ///
    /// A `Result` that contains a boolean indicating whether all the services are online or an
    /// `SMDBError` on failure.
    ///
    async fn check_if_services_online(&self, services: Vec<ServiceID>) -> Result<bool, SMDBError>;
    /// Sets a service with the given ID online in the SMDB.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to set online.
    ///
    /// # Returns
    ///
    /// A `Result` that contains a boolean indicating whether the service was successfully set online or an
    /// `SMDBError` on failure.
    ///
    async fn set_service_online(&self, id: ServiceID) -> Result<bool, SMDBError>;
    /// Sets a service with the given ID offline in the SMDB.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to set offline.
    ///
    /// # Returns
    ///
    /// A `Result` that contains a boolean indicating whether the service was successfully set offline or an
    /// `SMDBError` on failure.
    ///
    async fn set_service_offline(&self, id: ServiceID) -> Result<bool, SMDBError>;
}
