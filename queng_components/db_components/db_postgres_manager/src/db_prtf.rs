use common_exchange::prelude::{Instrument, PortfolioConfig};
use common_pg_queries::{pg_inserts, pg_query, pg_update};

use crate::error::PostgresDBError;
use crate::PostgresDBManager;

const DEFAULT_SCHEMA: &str = "public";

const PORTFOLIO_TABLE: &str = "portfolio";

impl PostgresDBManager {
    /// Inserts a new portfolio config into the database.
    ///
    /// # Arguments
    ///
    /// * `data` - The portfolio config to insert.
    ///
    /// # Returns
    ///
    /// Returns `()` on success, or a `PostgresDBError` on failure.
    ///
    pub async fn insert_portfolio_config(
        &self,
        data: &PortfolioConfig,
    ) -> Result<(), PostgresDBError> {
        self.dbg_print("insert_portfolio");

        self.dbg_print("[insert_portfolio]: insert portfolio");
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

        //
        // Update to PostgreSQL INSERT IF NOT EXISTS
        //
        self.dbg_print("[insert_portfolio]: insert instruments");
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

            self.dbg_print("[insert_portfolio]: insert_portfolio_instrument");
            self.dbg_print(&format!("Portfolio id: {}", portfolio_id));
            self.dbg_print(&format!("Instrument id: {}", instrument_id));
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

    // Change to PostgreSQL INSERT IF NOT EXISTS
    // https://www.commandprompt.com/education/postgresql-insert-if-not-exists/

    async fn insert_instrument(&self, data: &Instrument) -> Result<String, PostgresDBError> {
        self.dbg_print("insert_instrument");

        let instrument_id = data.code();
        let exists = match self.check_if_instrument_id_exists(instrument_id).await {
            Ok(exists) => exists,
            Err(err) => {
                return Err(PostgresDBError::InsertFailed(format!(
                    "Failed to check if instrument exists: {}",
                    err
                )))
            }
        };

        if exists {
            return Ok(instrument_id.to_string());
        }

        let query = pg_inserts::build_insert_instrument_query(data);
        match self.client.query_one(&query, &[]).await {
            Ok(row) => {
                let code = row.get::<usize, String>(0);
                Ok(code)
            }
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
        instrument_id: String,
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
    /// returns the portfolio config with the given id
    ///
    /// # Arguments
    ///
    /// * `portfolio_id` - The ID of the portfolio to read.
    ///
    /// # Returns
    ///
    /// * `Result<Option<PortfolioConfig>, PostgresDBError>` -
    ///   A result indicating success or failure.
    ///   If successful, returns a `Some(PortfolioConfig)` with the portfolio data.
    ///   If the portfolio does not exist, returns `Ok(None)`.
    ///
    pub async fn read_portfolio_config_by_id(
        &self,
        portfolio_id: u16,
    ) -> Result<Option<PortfolioConfig>, PostgresDBError> {
        self.dbg_print("read_portfolio_config_by_id");

        // Check if the portfolio exists
        self.dbg_print("Check if portfolio exists");
        match self.check_if_portfolio_id_exists(portfolio_id).await {
            Ok(exists) => {
                self.dbg_print(&format!("Portfolio exists: {}", exists));
                if !exists {
                    return Ok(None);
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

        // First, we have to query for all the instruments id's that match the given portfolio_id
        self.dbg_print("Query for portfolio_instrument");
        let query_instrument_ids_ =
            pg_query::build_query_instrument_ids_by_portfolio_id(portfolio_id);

        let instrument_ids = match self.client.query(&query_instrument_ids_, &[]).await {
            Ok(res) => {
                let mut instrument_ids = Vec::new();
                for row in res {
                    let instrument_id: String = row.get(0);
                    instrument_ids.push(instrument_id);
                }
                instrument_ids
            }
            Err(e) => {
                self.dbg_print(&format!("Query failed: \n {}", query_instrument_ids_));
                return Err(PostgresDBError::QueryFailed(e.to_string()));
            }
        };

        self.dbg_print(&format!("Instrument ID's {:?}", &instrument_ids.as_slice()));

        // Then we have to fetch all the instruments for the portfolio
        self.dbg_print("Query for instruments");
        let query_instruments = pg_query::build_query_instruments_by_ids(&instrument_ids);
        let instruments = match self.client.query(&query_instruments, &[]).await {
            Ok(res) => {
                let mut instruments = Vec::new();
                for row in res {
                    let instrument = Instrument::from_sql_row(&row);
                    instruments.push(instrument);
                }
                instruments
            }
            Err(e) => {
                self.dbg_print(&format!("Query failed: \n {}", query_instruments));
                return Err(PostgresDBError::QueryFailed(e.to_string()));
            }
        };

        // Then we have to fetch the portfolio
        self.dbg_print("Query for portfolio");
        let query = pg_query::build_query_portfolio_by_id(portfolio_id);
        match self.client.query_one(&query, &[]).await {
            Ok(row) => {
                let portfolio = PortfolioConfig::from_sql_row(&row, instruments);
                Ok(Some(portfolio))
            }
            Err(e) => {
                self.dbg_print(&format!("Query failed: \n {}", query));
                Err(PostgresDBError::QueryFailed(e.to_string()))
            }
        }
    }

    pub async fn read_all_portfolio_configs(
        &self,
    ) -> Result<Vec<PortfolioConfig>, PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }

    /// Returns the number of portfolio configurations in the database.
    ///
    /// # Returns
    ///
    /// * `Result<u64, PostgresDBError>` - A result indicating success or failure.
    /// The number of portfolio configurations is returned as a `u64` if successful.
    ///
    pub async fn count_portfolio_config(&self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_portfolios");

        match self
            .execute_count_query(DEFAULT_SCHEMA, PORTFOLIO_TABLE)
            .await
        {
            Ok(count) => Ok(count),
            Err(e) => Err(e),
        }
    }

    /// Checks if a portfolio configuration with the given ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `portfolio_id` - The ID of the portfolio configuration to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, PostgresDBError>` - A result indicating success or failure.
    /// If the portfolio configuration exists, returns `Ok(true)`, otherwise `Ok(false)`.
    ///
    /// # Errors
    ///
    /// Returns an PostgresDBError error if the query fails.
    pub async fn check_if_portfolio_id_exists(
        &self,
        portfolio_id: u16,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_portfolio_id_exists");

        let query = pg_query::build_check_if_portfolio_id_exists_query(portfolio_id);
        match self.execute_exists_query(&query).await {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    /// Checks if an instrument with the given ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `instrument_id` - The ID of the instrument to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, PostgresDBError>` - A result indicating success or failure.
    /// If the instrument exists, returns `Ok(true)`, otherwise `Ok(false)`.
    ///
    /// # Errors
    ///
    /// Returns an PostgresDBError error if the query fails.
    ///
    pub async fn check_if_instrument_id_exists(
        &self,
        instrument_id: &str,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_instrument_id_exists");

        let query = pg_query::build_check_if_instrument_id_exists_query(instrument_id);
        match self.execute_exists_query(&query).await {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    /// updates the portfolio config with the given data
    /// Updates the portfolio config with the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - The portfolio config to update.
    ///
    /// # Returns
    ///
    /// * `Result<Option<PortfolioConfig>, PostgresDBError>` - A result indicating success or failure.
    /// If the portfolio was updated, returns `Ok(Some(PortfolioConfig))`.
    /// If the portfolio does not exist, returns `Ok(None)`.
    ///
    pub async fn update_portfolio_config(
        &self,
        data: PortfolioConfig,
    ) -> Result<Option<PortfolioConfig>, PostgresDBError> {
        self.dbg_print("update_portfolio_config");

        let query = pg_update::build_update_portfolio_query(&data);
        match self.execute_query(&query).await {
            Ok(_) => Ok(Some(data)),
            Err(e) => Err(e),
        }
    }

    /// Deletes a portfolio from the database by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the portfolio to delete.
    ///
    /// # Returns
    ///
    /// * `Result<bool, PostgresDBError>` - A result indicating success or failure.
    /// If the portfolio was deleted, returns `Ok(true)`.
    /// If the portfolio does not exist, returns `Ok(false)`.
    ///
    pub async fn delete_portfolio_config(&self, id: u16) -> Result<bool, PostgresDBError> {
        self.dbg_print("delete_portfolio_config");

        match self.check_if_portfolio_id_exists(id).await {
            Ok(exists) => {
                if !exists {
                    return Ok(false);
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

        self.dbg_print("delete instruments associated with portfolio");
        let query = pg_query::build_delete_portfolio_instrument_query(id);
        match self.execute_query(&query).await {
            Ok(_) => (),
            Err(e) => {
                return Err(e);
            }
        }

        self.dbg_print("delete actual portfolio");
        let query = pg_query::build_delete_portfolio_query(id);
        match self.execute_query(&query).await {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
}
