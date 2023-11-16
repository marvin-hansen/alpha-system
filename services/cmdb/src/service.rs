use tarpc::context::Context;

use common::prelude::CMDBError;
use dbgw_client::DBGatewayClient;

#[tarpc::service]
pub trait CMDBService {
    async fn check_ok() -> Result<bool, CMDBError>;
}

#[derive(Clone)]
pub struct CMDBServer {
    dbgw: DBGatewayClient,
}

impl CMDBServer {
    pub fn new(dbgw: DBGatewayClient) -> Self {
        Self { dbgw }
    }
}

#[tarpc::server]
impl CMDBService for CMDBServer {
    async fn check_ok(self, _: Context) -> Result<bool, CMDBError> {
        Ok(true)
    }
}
