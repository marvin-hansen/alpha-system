use crate::PostgresCMDBManager;
use common_errors::PostgresDBError;
use common_exchange::PortfolioConfig;
use pg_cmdb::model::instrument::Instrument;
use pg_cmdb::portfolio::Portfolio;

impl PostgresCMDBManager {
    /// Inserts a portfolio configuration into the database.
    ///
    /// # Arguments
    ///
    /// * `self` - The `PostgresCMDBManager` instance.
    /// * `data` - The `PortfolioConfig` to insert.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `()` if the portfolio configuration was successfully inserted,
    /// or an `Err` containing a `PostgresDBError` if there was an error inserting the portfolio configuration.
    ///
    pub async fn insert_portfolio_config(
        &self,
        data: &PortfolioConfig,
    ) -> Result<PortfolioConfig, PostgresDBError> {
        self.dbg_print("insert_portfolio_config");
        let conn = &mut self.get_connection();

        match Portfolio::create(conn, data) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    ///
    /// Asynchronously inserts a collection of portfolio configurations into the database.
    ///
    /// # Arguments
    /// - `data`: A slice of `PortfolioConfig` containing the portfolio configurations to be inserted.
    ///
    /// # Returns
    /// - `Result<(), PostgresDBError>`
    ///
    pub async fn insert_portfolio_config_collection(
        &self,
        data: &[PortfolioConfig],
    ) -> Result<(), PostgresDBError> {
        self.dbg_print("insert_portfolio_config_collection");
        let conn = &mut self.get_connection();

        match Portfolio::create_portfolio_collection(conn, data) {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    /// Returns the number of portfolio configurations in the database.
    ///
    /// # Returns
    ///
    /// * `Result<u64, PostgresDBError>` - A result indicating success or failure.
    ///
    /// The number of portfolio configurations is returned as a `u64` if successful.
    ///
    pub async fn count_portfolio_configs(&self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_portfolio_configs");
        let conn = &mut self.get_connection();

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
    ///
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
        let conn = &mut self.get_connection();

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
    ///
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
        let conn = &mut self.get_connection();

        match Instrument::check_if_instrument_code_exists(conn, instrument_id.to_string()) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        }
    }

    /// Reads a portfolio configuration from the database by ID.
    ///
    /// # Arguments
    ///
    /// * `self` - The `PostgresCMDBManager` instance.
    /// * `portfolio_id` - The ID of the portfolio configuration to read.
    ///
    /// # Returns
    ///
    /// * `Result` containing a `PortfolioConfig` if the portfolio configuration was successfully read,
    /// * `Err` containing a `PostgresDBError` if there was an error reading the portfolio configuration.
    ///
    pub async fn read_portfolio_config_by_id(
        &self,
        portfolio_id: u16,
    ) -> Result<PortfolioConfig, PostgresDBError> {
        self.dbg_print("read_portfolio_config_by_id");
        let conn = &mut self.get_connection();

        self.dbg_print("check_if_portfolio_id_exists");
        match Portfolio::read(conn, portfolio_id as i32) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Reads all portfolio configurations from the database.
    ///
    /// # Returns
    ///
    /// * `Result` containing a `Vec` of `PortfolioConfig` if the portfolio configurations were successfully read,
    /// * `Err` containing a `PostgresDBError` if there was an error reading the portfolio configurations.
    ///
    /// Notice, if there are no data to read, the vector will be of length 0.
    ///
    pub async fn read_all_portfolio_configs(
        &self,
    ) -> Result<Vec<PortfolioConfig>, PostgresDBError> {
        self.dbg_print("read_all_portfolio_configs");
        let conn = &mut self.get_connection();

        match Portfolio::read_all(conn) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Updates a portfolio configuration in the database.
    ///
    /// # Arguments
    ///
    /// * `self` - The `PostgresCMDBManager` instance.
    /// * `data` - The `PortfolioConfig` containing the new data for the portfolio configuration.
    ///
    /// # Returns
    ///
    /// * `Result` containing a `()` if the portfolio configuration was successfully updated,
    /// * `Err` containing a `PostgresDBError` if there was an error updating the portfolio configuration.
    ///
    pub async fn update_portfolio_config(
        &self,
        data: PortfolioConfig,
    ) -> Result<(), PostgresDBError> {
        self.dbg_print("update_portfolio_config");
        let conn = &mut self.get_connection();

        let portfolio_id = data.portfolio_id() as i32;
        match Portfolio::update(conn, portfolio_id, &data) {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresDBError::UpdateFailed(e.to_string())),
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
    /// * If the portfolio was deleted, returns `Ok(true)`.
    /// * If the portfolio does not exist, returns `Ok(false)`.
    /// * If the deletion fails, returns an `Err` containing a `PostgresDBError`.
    ///
    pub async fn delete_portfolio_config(&self, id: u16) -> Result<bool, PostgresDBError> {
        self.dbg_print("delete_portfolio_config");
        let conn = &mut self.get_connection();

        match self.check_if_portfolio_id_exists(id).await {
            Ok(exists) => {
                if !exists {
                    Ok(false)
                } else {
                    match Portfolio::delete(conn, id as i32) {
                        Ok(_) => Ok(true),
                        Err(e) => Err(PostgresDBError::DeleteFailed(e.to_string())),
                    }
                }
            }
            Err(e) => Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        }
    }
}
