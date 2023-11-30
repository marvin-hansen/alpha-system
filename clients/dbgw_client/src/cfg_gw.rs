use tarpc::context;

use common::prelude::PortfolioConfig;
use dbgw_service::service_db::DBGatewayError;

use crate::DBGatewayClient;

impl DBGatewayClient {
    pub async fn create_portfolio_config(
        self,
        data: PortfolioConfig,
    ) -> Result<bool, DBGatewayError> {
        let res = self
            .client
            .create_portfolio_config(context::current(), data)
            .await
            .expect("Failed to create portfolio config");

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn read_all_portfolio_configs(self) -> Result<Vec<PortfolioConfig>, DBGatewayError> {
        let res = self
            .client
            .read_all_portfolio_configs(context::current())
            .await
            .expect("Failed to read all portfolio configs");

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn read_portfolio_config_by_id(
        self,
        id: u16,
    ) -> Result<Option<PortfolioConfig>, DBGatewayError> {
        let res = self
            .client
            .read_portfolio_config_by_id(context::current(), id)
            .await
            .expect("Failed to read portfolio config by id");

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn update_portfolio_config(
        self,
        data: PortfolioConfig,
    ) -> Result<Option<PortfolioConfig>, DBGatewayError> {
        let res = self
            .client
            .update_portfolio_config(context::current(), data)
            .await
            .expect("Failed to update portfolio config");

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_portfolio_config(self, id: u16) -> Result<bool, DBGatewayError> {
        let res = self
            .client
            .delete_portfolio_config(context::current(), id)
            .await
            .expect("Failed to delete portfolio config");

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }
}
