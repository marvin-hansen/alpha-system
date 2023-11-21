use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};
use tarpc::context::Context;

use common::prelude::PortfolioConfig;
use dbgw_client::DBGatewayClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct CMDBError(pub String);

impl Error for CMDBError {}

impl fmt::Display for CMDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CMDBError: {}", self.0)
    }
}


#[tarpc::service]
pub trait CMDBService {
    async fn create_portfolio_config(config: PortfolioConfig) -> Result<bool, CMDBError>;
    async fn read_all_portfolio_configs() -> Result<Vec<PortfolioConfig>, CMDBError>;
    async fn read_portfolio_config_by_id(id: u16) -> Result<Option<PortfolioConfig>, CMDBError>;
    async fn update_portfolio_config(data: PortfolioConfig) -> Result<Option<PortfolioConfig>, CMDBError>;
    async fn delete_portfolio_config(id: u16) -> Result<bool, CMDBError>;
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
    async fn create_portfolio_config(
        self,
        _: Context,
        config: PortfolioConfig,
    ) -> Result<bool, CMDBError> {
        let res = self.dbgw.create_portfolio_config(config).await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(CMDBError(e.to_string())),
        }
    }

    async fn read_all_portfolio_configs(
        self,
        _: Context,
    ) -> Result<Vec<PortfolioConfig>, CMDBError> {
        let res = self.dbgw.read_all_portfolio_configs().await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(CMDBError(e.to_string())),
        }
    }

    async fn read_portfolio_config_by_id(
        self,
        _: Context,
        id: u16,
    ) -> Result<Option<PortfolioConfig>, CMDBError> {
        let res = self.dbgw.read_portfolio_config_by_id(id).await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(CMDBError(e.to_string())),
        }
    }

    async fn update_portfolio_config(
        self,
        _: Context,
        data: PortfolioConfig,
    ) -> Result<Option<PortfolioConfig>, CMDBError> {
        let res = self.dbgw.update_portfolio_config(data).await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(CMDBError(e.to_string())),
        }
    }

    async fn delete_portfolio_config(
        self,
        _: Context,
        id: u16,
    ) -> Result<bool, CMDBError> {
        let res = self.dbgw.delete_portfolio_config(id).await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(CMDBError(e.to_string())),
        }
    }
}
