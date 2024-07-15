use crate::error::SurrealDBError;
use crate::SurrealDBManager;
use common_config::prelude::ServiceConfig;
use common_exchange::prelude::PortfolioConfig;

const PORTFOLIO_TABLE: &str = "portfolio";

impl SurrealDBManager {
    /// add_portfolio_config that adds a PortfolioConfig config to the database
    /// Returns true in case of success, or false in case of insert error,
    /// or an error in case of a database error.
    pub async fn insert_portfolio_config(
        &self,
        config: &PortfolioConfig,
    ) -> Result<(), SurrealDBError> {
        let table = PORTFOLIO_TABLE;
        let id = config.portfolio_id().to_string();

        let created: Option<PortfolioConfig> = self
            .db
            .update((table, id))
            .merge(config)
            .await
            .expect("Failed to create portfolio config");

        match created {
            Some(_) => Ok(()),
            None => Err(SurrealDBError::InsertFailed("No data inserted".to_string())),
        }
    }

    /// returns all the portfolio configs in the database
    pub async fn read_all_portfolio_configs(&self) -> Result<Vec<PortfolioConfig>, SurrealDBError> {
        let res = match self.db.select(PORTFOLIO_TABLE).await {
            Ok(res) => res,
            Err(e) => return Err(SurrealDBError::UpdateFailed(e.to_string())),
        };

        Ok(res)
    }

    /// returns the portfolio config with the given id
    pub async fn read_portfolio_config_by_id(
        &self,
        id: u16,
    ) -> Result<Option<PortfolioConfig>, SurrealDBError> {
        let id = id.to_string();

        let res = match self.db.select((PORTFOLIO_TABLE, id)).await {
            Ok(res) => res,
            Err(e) => return Err(SurrealDBError::UpdateFailed(e.to_string())),
        };

        Ok(res)
    }

    pub async fn count_portfolio_config(&self) -> Result<u64, SurrealDBError> {
        let res: Vec<ServiceConfig> = match self.db.select(PORTFOLIO_TABLE).await {
            Ok(res) => res,
            Err(e) => return Err(SurrealDBError::QueryFailed(e.to_string())),
        };

        let count = res.len() as u64;

        Ok(count)
    }

    /// updates the portfolio config with the given data
    pub async fn update_portfolio_config(
        &self,
        data: PortfolioConfig,
    ) -> Result<Option<PortfolioConfig>, SurrealDBError> {
        let id = data.portfolio_id().to_string();

        let update = match self.db.update((PORTFOLIO_TABLE, id)).content(data).await {
            Ok(res) => res,
            Err(e) => return Err(SurrealDBError::UpdateFailed(e.to_string())),
        };

        Ok(update)
    }
    /// deletes the portfolio config with the given id
    pub async fn delete_portfolio_config(&self, id: u16) -> Result<bool, SurrealDBError> {
        let id = id.to_string();

        let deleted: Option<PortfolioConfig> = match self.db.delete((PORTFOLIO_TABLE, id)).await {
            Ok(res) => res,
            Err(e) => return Err(SurrealDBError::DeleteFailed(e.to_string())),
        };

        match deleted {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
