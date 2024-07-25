use common_exchange::prelude::{Instrument, PortfolioConfig};
use common_pg_queries::pg_inserts;

use crate::error::PostgresDBError;
use crate::PostgresDBManager;

// const PORTFOLIO_TABLE: &str = "portfolio";

impl PostgresDBManager {
    /// add_portfolio_config that adds a PortfolioConfig config to the database
    /// Returns true in case of success, or false in case of insert error,
    /// or an error in case of a database error.
    pub async fn insert_portfolio_config(
        &self,
        data: &PortfolioConfig,
    ) -> Result<(), PostgresDBError> {
        self.dbg_print("insert_portfolio");

        let query = pg_inserts::build_insert_portfolio_query(data);
        let portfolio_id = match self.execute_insert_query(&query).await {
            Ok(id) => id,
            Err(err) => {
                return Err(PostgresDBError::InsertFailed(format!(
                    "Failed to insert portfolio: {}",
                    err
                )))
            }
        };

        for instrument in data.portfolio_instruments() {
            let instrument_id = match self.insert_instrument(instrument).await {
                Ok(id) => id,
                Err(err) => {
                    return Err(PostgresDBError::InsertFailed(format!(
                        "Failed to insert instrument: {}",
                        err
                    )))
                }
            };

            match self
                .insert_portfolio_instrument(portfolio_id, instrument_id)
                .await
            {
                Ok(_) => (),
                Err(err) => {
                    return Err(PostgresDBError::InsertFailed(format!(
                        "Failed to insert portfolio_instrument: {}",
                        err
                    )))
                }
            };
        }

        Ok(())
    }

    async fn insert_instrument(&self, data: &Instrument) -> Result<u64, PostgresDBError> {
        self.dbg_print("insert_instrument");

        let query = pg_inserts::build_insert_instrument_query(data);
        // println!("query: {}", query);
        match self.execute_insert_query(&query).await {
            Ok(id) => Ok(id),
            Err(err) => Err(PostgresDBError::InsertFailed(format!(
                "Failed to insert instrument: {} due error: {}",
                &data.code(),
                err
            ))),
        }
    }

    async fn insert_portfolio_instrument(
        &self,
        portfolio_id: u64,
        instrument_id: u64,
    ) -> Result<(), PostgresDBError> {
        self.dbg_print("insert_portfolio_instrument");

        let query =
            pg_inserts::build_insert_portfolio_instrument_query(portfolio_id, instrument_id);
        // println!("query: {}", query);
        match self.execute_query(&query).await {
            Ok(_) => Ok(()),
            Err(err) => Err(PostgresDBError::InsertFailed(format!(
                "Failed to insert portfolio_instrument due error: {}",
                err
            ))),
        }
    }
}

impl PostgresDBManager {
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
