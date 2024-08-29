use crate::PostgresCMDBManager;
use common_errors::prelude::PostgresDBError;
use common_exchange::prelude::PortfolioConfig;
use diesel::Connection;
use pg_cmdb::model::instrument::Instrument;
use pg_cmdb::model::portfolio_instrument::PortfolioInstrument;
use pg_cmdb::prelude::portfolio::Portfolio;

impl PostgresCMDBManager {
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
        self.dbg_print("insert_portfolio_config");
        let conn = &mut self.pool.get().unwrap();

        match Portfolio::create(conn, data) {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    /// Returns the number of portfolio configurations in the database.
    ///
    /// # Returns
    ///
    /// * `Result<u64, PostgresDBError>` - A result indicating success or failure.
    /// The number of portfolio configurations is returned as a `u64` if successful.
    ///
    pub async fn count_portfolio_configs(&mut self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_portfolio_configs");
        let conn = &mut self.pool.get().unwrap();

        match Portfolio::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
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
    ///
    pub async fn check_if_portfolio_id_exists(
        &self,
        portfolio_id: u16,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_portfolio_id_exists");
        let conn = &mut self.pool.get().unwrap();

        match Portfolio::check_if_portfolio_id_exists(conn, portfolio_id as i32) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
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
        let conn = &mut self.pool.get().unwrap();

        match Instrument::check_if_instrument_code_exists(conn, instrument_id.to_string()) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        }
    }

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
        let conn = &mut self.pool.get().unwrap();

        let portfolio_id = data.portfolio_id() as i32;
        match Portfolio::update(conn, portfolio_id, &data) {
            Ok(_) => Ok(Some(data)),
            Err(e) => Err(PostgresDBError::UpdateFailed(e.to_string())),
        }
    }

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
        let conn = &mut self.pool.get().unwrap();

        self.dbg_print("check_if_portfolio_id_exists");
        match Portfolio::check_if_portfolio_id_exists(conn, portfolio_id as i32) {
            Ok(exists) => {
                if !exists {
                    return Ok(None);
                };
            }
            Err(e) => return Err(PostgresDBError::CheckFailed(e.to_string())),
        };

        //  Implement later
        Ok(None)
    }

    pub async fn read_all_portfolio_configs(
        &self,
    ) -> Result<Vec<PortfolioConfig>, PostgresDBError> {
        self.dbg_print("read_all_portfolio_configs");

        //  Implement later
        Ok(Vec::new())
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
        let conn = &mut self.pool.get().unwrap();

        self.dbg_print("check_if_portfolio_id_exists");
        match Portfolio::check_if_portfolio_id_exists(conn, id as i32) {
            Ok(exists) => {
                if !exists {
                    return Ok(false);
                }
            }
            Err(e) => return Err(PostgresDBError::CheckFailed(e.to_string())),
        }

        // Start transaction
        match conn.transaction(|db| {
            // Read all portfolio_instrument for portfolio
            let portfolio_instruments =
                match PortfolioInstrument::read_instruments_for_portfolio(db, id as i32) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

            // Delete portfolio
            let res = match Portfolio::delete(db, id as i32) {
                Ok(res) => res,
                Err(e) => return Err(e),
            };

            // Delete all portfolio_instrument for portfolio
            for i in portfolio_instruments {
                match PortfolioInstrument::delete(db, i.portfolio_id, i.instrument_id) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                };
            }

            Ok(res)
        }) {
            Ok(_) => Ok(true),
            Err(e) => Err(PostgresDBError::DeleteFailed(e.to_string())),
        }
    }
}
