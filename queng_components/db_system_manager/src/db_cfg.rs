use crate::SystemDBManager;
use common::prelude::PortfolioConfig;
use std::fmt::Error;

impl SystemDBManager {
    pub async fn add_portfolio_config(&self, config: &PortfolioConfig) -> Result<bool, Error> {
        println!("{:?}", config);

        // Check if id is in portfolio_cache, if so remove entry from cache.
        if self
            .portfolio_cache
            .read()
            .unwrap()
            .contains_key(&config.portfolio_id())
        {
            self.portfolio_cache
                .write()
                .expect("Failed to acquire write lock")
                .remove(&config.portfolio_id());
        }

        // Otherwise, add new portfolio config to portfolio_cache
        self.portfolio_cache
            .write()
            .expect("Failed to acquire write lock")
            .insert(config.portfolio_id(), config.clone());

        // Check if there is an id in the database, if so update database.

        Ok(true)
    }

    pub async fn read_all_portfolio_configs(&self) -> Result<Vec<PortfolioConfig>, Error> {
        // Check if the cache is empty,
        if self.portfolio_cache.read().unwrap().is_empty() {
            // if so query the database to get all portfolio configs,
            // add them to cache and return.
        }

        // otherwise return portfolio_cache as vector
        let portfolio = self
            .portfolio_cache
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<PortfolioConfig>>();

        Ok(portfolio)
    }

    pub async fn read_portfolio_config_by_id(
        &self,
        id: u16,
    ) -> Result<Option<PortfolioConfig>, Error> {
        println!("{}", id);

        let id = id as u32;

        // Check if id is in portfolio_cache, if so return,
        if self.portfolio_cache.read().unwrap().contains_key(&id) {
            let portfolio = self
                .portfolio_cache
                .read()
                .unwrap()
                .get(&id)
                .unwrap()
                .clone();
            return Ok(Some(portfolio));
        }
        // otherwise query the database, add to portfolio_cache, and return.

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

        let id = id as u32;

        // Check if id is in portfolio_cache, if so delete from cache.
        if self.portfolio_cache.read().unwrap().contains_key(&id) {
            self.portfolio_cache
                .write()
                .expect("Failed to acquire write lock")
                .remove(&id);
        }

        // Otherwise, check if id is in database, if so delete from database.

        Ok(true)
    }
}
