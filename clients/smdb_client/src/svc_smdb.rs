use tarpc::context;

use common::prelude::{SMDBError, ServiceID};

use crate::SMDBClient;

impl SMDBClient {
    async fn check_if_service_id_exists(self, id: ServiceID) -> Result<bool, SMDBError> {
        let res = self
            .client
            .check_if_service_id_exists(context::current(), id)
            .await
            .expect("RPC call failed to check if service id exists");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    async fn check_if_services_exists(self, services: Vec<ServiceID>) -> Result<bool, SMDBError> {
        let res = self
            .client
            .check_if_services_exists(context::current(), services)
            .await
            .expect("RPC call failed to check if services exists");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    async fn set_service_online(self, id: ServiceID) -> Result<bool, SMDBError> {
        let res = self
            .client
            .set_service_online(context::current(), id)
            .await
            .expect("RPC call failed to set service online");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    async fn check_if_service_id_online(self, id: ServiceID) -> Result<bool, SMDBError> {
        let res = self
            .client
            .check_if_service_id_online(context::current(), id)
            .await
            .expect("RPC call failed to check if service id online");

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    async fn check_if_services_online(self, id: Vec<ServiceID>) -> Result<bool, SMDBError> {
        let res = self
            .client
            .check_if_services_online(context::current(), id)
            .await
            .expect("Failed to check if services online");

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    async fn set_service_offline(self, id: ServiceID) -> Result<bool, SMDBError> {
        let res = self
            .client
            .set_service_offline(context::current(), id)
            .await
            .expect("RPC call failed to set service online");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }
}
