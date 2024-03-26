use crate::SystemDBManager;
use common::prelude::PortfolioConfig;
use std::fmt::Error;

impl SystemDBManager {
    pub async fn add_portfolio_config(&self, config: &PortfolioConfig) -> Result<bool, Error> {
        println!("{:?}", config);

        Ok(true)
    }

    pub async fn read_all_portfolio_configs(&self) -> Result<Vec<PortfolioConfig>, Error> {
        Ok(Vec::new())
    }

    pub async fn read_portfolio_config_by_id(
        &self,
        id: u16,
    ) -> Result<Option<PortfolioConfig>, Error> {
        println!("{}", id);

        Ok(None)
    }

    pub async fn update_portfolio_config(
        &self,
        data: PortfolioConfig,
    ) -> Result<Option<PortfolioConfig>, Error> {
        println!("{:?}", data);

        Ok(None)
    }

    pub async fn delete_portfolio_config(&self, id: u16) -> Result<bool, Error> {
        println!("{}", id);

        Ok(true)
    }
}
