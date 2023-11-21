use crate::prelude::DBManager;
use common::prelude::PortfolioConfig;
use surrealdb::Error;

const PORTFOLIO_CONFIG_TABLE: &str = "portfolio_config";

impl DBManager {
    /// add_portfolio_config that adds a PortfolioConfig config to the database
    /// Returns true in case of success, or false in case of insert error,
    /// or an error in case of a database error.
    pub async fn add_portfolio_config(&self, config: &PortfolioConfig) -> Result<bool, Error> {
        let table = PORTFOLIO_CONFIG_TABLE;
        let id = config.portfolio_id().to_string();

        let created: Option<PortfolioConfig> = self
            .db
            .update((table, id))
            .merge(config)
            .await
            .expect("Failed to create portfolio config");

        match created {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    /// returns all the portfolio configs in the database
    pub async fn read_all_portfolio_configs(&self) -> Result<Vec<PortfolioConfig>, Error> {
        let res = self
            .db
            .select(PORTFOLIO_CONFIG_TABLE)
            .await
            .expect("Failed to read all portfolio configs");

        Ok(res)
    }

    /// returns the portfolio config with the given id
    pub async fn read_portfolio_config_by_id(
        &self,
        id: u16,
    ) -> Result<Option<PortfolioConfig>, Error> {
        let id = id.to_string();

        let res = self
            .db
            .select((PORTFOLIO_CONFIG_TABLE, id))
            .await
            .expect("Failed to read portfolio config by id");

        Ok(res)
    }

    /// updates the portfolio config with the given data
    pub async fn update_portfolio_config(
        &self,
        data: PortfolioConfig,
    ) -> Result<Option<PortfolioConfig>, Error> {
        let id = data.portfolio_id().to_string();

        let updated = self
            .db
            .update((PORTFOLIO_CONFIG_TABLE, id))
            .content(data)
            .await
            .expect("Failed to update portfolio config");

        Ok(updated)
    }
    /// deletes the portfolio config with the given id
    pub async fn delete_portfolio_config(&self, id: u16) -> Result<bool, Error> {
        let id = id.to_string();

        let deleted: Option<PortfolioConfig> = self
            .db
            .delete((PORTFOLIO_CONFIG_TABLE, id))
            .await
            .expect("Failed to delete portfolio config");

        match deleted {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
