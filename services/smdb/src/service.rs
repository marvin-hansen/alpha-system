use tarpc::context::Context;

use common::errors::SMDBError;
use common::prelude::ServiceID;
use dbgw_client::DBGatewayClient;

#[tarpc::service]
pub trait SMDBService {
    async fn check_if_service_id_exists(id: ServiceID) -> Result<bool, SMDBError>;
    async fn check_if_services_exists(services: Vec<ServiceID>) -> Result<bool, SMDBError>;
    async fn set_service_online(id: ServiceID) -> Result<bool, SMDBError>;
    async fn set_service_offline(id: ServiceID) -> Result<bool, SMDBError>;
}

#[derive(Clone)]
pub struct SMDBServer {
    dbgw: DBGatewayClient,
}

impl SMDBServer {
    pub fn new(dbgw: DBGatewayClient) -> Self {
        Self { dbgw }
    }
}

#[tarpc::server]
impl SMDBService for SMDBServer {
    async fn check_if_service_id_exists(self, _: Context, id: ServiceID) -> Result<bool, SMDBError> {
        Ok(true)
    }

    async fn check_if_services_exists(self, _: Context, services: Vec<ServiceID>) -> Result<bool, SMDBError> {
        Ok(true)
    }
    async fn set_service_online(self, _: Context, id: ServiceID) -> Result<bool, SMDBError> {
        Ok(true)
    }
    async fn set_service_offline(self, _: Context, id: ServiceID) -> Result<bool, SMDBError> {
        Ok(true)
    }
}