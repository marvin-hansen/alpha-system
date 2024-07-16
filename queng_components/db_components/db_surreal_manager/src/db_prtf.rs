use crate::error::PostgresDBError;
use crate::SurrealDBManager;
use common_exchange::prelude::PortfolioConfig;

// const PORTFOLIO_TABLE: &str = "portfolio";

impl SurrealDBManager {
    /// add_portfolio_config that adds a PortfolioConfig config to the database
    /// Returns true in case of success, or false in case of insert error,
    /// or an error in case of a database error.
    pub async fn insert_portfolio_config(
        &self,
        _config: &PortfolioConfig,
    ) -> Result<(), PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }

    /// returns all the portfolio configs in the database
    pub async fn read_all_portfolio_configs(
        &self,
    ) -> Result<Vec<PortfolioConfig>, PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }

    /// returns the portfolio config with the given id
    pub async fn read_portfolio_config_by_id(
        &self,
        _id: u16,
    ) -> Result<Option<PortfolioConfig>, PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }

    pub async fn count_portfolio_config(&self) -> Result<u64, PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }

    /// updates the portfolio config with the given data
    pub async fn update_portfolio_config(
        &self,
        _data: PortfolioConfig,
    ) -> Result<Option<PortfolioConfig>, PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }
    /// deletes the portfolio config with the given id
    pub async fn delete_portfolio_config(&self, _id: u16) -> Result<bool, PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }
}
