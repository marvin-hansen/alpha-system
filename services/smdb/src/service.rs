use tarpc::context::Context;

use common::prelude::{SMDBError, ServiceID};
use dbgw_client::DBGatewayClient;

#[tarpc::service]
pub trait SMDBService {
    async fn check_if_service_id_exists(id: ServiceID) -> Result<bool, SMDBError>;
    async fn check_if_services_exists(services: Vec<ServiceID>) -> Result<bool, SMDBError>;
    async fn check_if_service_id_online(id: ServiceID) -> Result<bool, SMDBError>;
    async fn check_if_services_online(id: Vec<ServiceID>) -> Result<bool, SMDBError>;
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
    async fn check_if_service_id_exists(
        self,
        _: Context,
        id: ServiceID,
    ) -> Result<bool, SMDBError> {
        let res = self.dbgw.check_if_service_id_exists(id).await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    async fn check_if_services_exists(
        self,
        _: Context,
        services: Vec<ServiceID>,
    ) -> Result<bool, SMDBError> {
        let res = self.dbgw.check_if_services_exists(services).await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    async fn check_if_service_id_online(
        self,
        _: Context,
        id: ServiceID,
    ) -> Result<bool, SMDBError> {
        let res = self.dbgw.check_if_service_id_online(id).await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    async fn check_if_services_online(
        self,
        _: Context,
        id: Vec<ServiceID>,
    ) -> Result<bool, SMDBError> {
        let res = self.dbgw.check_if_services_online(id).await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    async fn set_service_online(self, _: Context, id: ServiceID) -> Result<bool, SMDBError> {
        let res = self.dbgw.set_service_online(id).await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }

    async fn set_service_offline(self, _: Context, id: ServiceID) -> Result<bool, SMDBError> {
        let res = self.dbgw.set_service_offline(id).await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(SMDBError(e.to_string())),
        }
    }
}
