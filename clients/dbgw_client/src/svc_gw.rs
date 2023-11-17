use tarpc::context;

use common::errors::DBGatewayError;
use common::prelude::{ServiceConfig, ServiceID};

use crate::DBGatewayClient;

impl DBGatewayClient {
    pub async fn create_service(&self, data: ServiceConfig) -> Result<bool, DBGatewayError> {
        let res = self
            .client
            .create_service(context::current(), data)
            .await
            .expect("RPC call failed to create service");
        
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn check_if_service_id_exists(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let res = self
            .client
            .check_if_service_id_exists(context::current(), id)
            .await
            .expect("RPC call failed to check if service id exists");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn check_if_services_exists(
        &self,
        services: Vec<ServiceID>,
    ) -> Result<bool, DBGatewayError> {
        let res = self
            .client
            .check_if_services_exists(context::current(), services)
            .await
            .expect("RPC call failed to check if services exists");

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn check_if_service_id_online(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let res = self
            .client
            .check_if_service_id_online(context::current(), id)
            .await
            .expect("RPC call failed to check if service id online");

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }
    pub async fn check_if_services_online(
        &self,
        id: Vec<ServiceID>,
    ) -> Result<bool, DBGatewayError> {
        let res = self
            .client
            .check_if_services_online(context::current(), id)
            .await
            .expect("Failed to check if services online");

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn read_all_services(&self) -> Result<Vec<ServiceConfig>, DBGatewayError> {
        let res = self
            .client
            .read_all_services(context::current())
            .await
            .expect("RPC call failed to read all services");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn read_service_by_id(&self, id: ServiceID) -> Result<ServiceConfig, DBGatewayError> {
        let res = self
            .client
            .read_service_by_id(context::current(), id)
            .await
            .expect("RPC call failed to read_service_by_id");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn set_service_online(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let res = self
            .client
            .set_service_online(context::current(), id)
            .await
            .expect("RPC call failed to set service online");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn set_service_offline(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let res = self
            .client
            .set_service_offline(context::current(), id)
            .await
            .expect("RPC call failed to set service online");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn update_service(&self, data: ServiceConfig) -> Result<bool, DBGatewayError> {
        let res = self
            .client
            .update_service(context::current(), data)
            .await
            .expect("RPC call failed to update service");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_service(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let res = self
            .client
            .delete_service(context::current(), id)
            .await
            .expect("RPC call failed to delete service");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }
}
