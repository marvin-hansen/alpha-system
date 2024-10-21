use crate::PostgresMDDBManager;
use common_errors::prelude::PostgresDBError;
use common_metadata::prelude::MetaExchange;
use pg_mddb::prelude::Exchange;

impl PostgresMDDBManager {
    /// Inserts a new exchange into the database.
    ///
    /// Args:
    ///     exchange (MetaExchange): A `MetaExchange` object representing the exchange to be inserted.
    ///
    /// Returns:
    ///     Result<MetaExchange, PostgresDBError>: The inserted `MetaExchange` on success.
    ///
    /// Raises:
    ///     PostgresDBError: If the insertion fails due to database errors.
    ///
    /// This method is asynchronous.
    pub async fn insert_exchange(
        &self,
        exchange: MetaExchange,
    ) -> Result<MetaExchange, PostgresDBError> {
        self.dbg_print("insert_exchange");
        let conn = &mut self.get_connection();

        match Exchange::create_exchange(conn, exchange) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    /// Inserts a collection of exchanges into the database.
    ///
    /// Args:
    ///     exchanges (&[MetaExchange]): A slice of `MetaExchange` objects representing the exchanges to be inserted.
    ///
    /// Returns:
    ///     Result<usize, PostgresDBError>: The number of inserted exchanges on success.
    ///
    /// Raises:
    ///     PostgresDBError: If the insertion fails due to database errors.
    ///
    /// This method is asynchronous.
    pub async fn insert_exchange_collection(
        &self,
        exchanges: &[MetaExchange],
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("insert_exchange_collection");
        let conn = &mut self.get_connection();

        match Exchange::create_exchange_collection(conn, exchanges) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    /// Counts the total number of exchanges in the database.
    ///
    /// Returns:
    ///     Result<u64, PostgresDBError>: The count of exchanges on success.
    ///
    /// Raises:
    ///     PostgresDBError: If the count operation fails due to database errors.
    ///
    /// This method is asynchronous.
    pub async fn count_exchanges(&self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_exchanges");
        let conn = &mut self.get_connection();

        match Exchange::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
        }
    }

    /// Checks if an exchange with the specified ID exists in the database.
    ///
    /// Args:
    ///     exchange_id_str (String): A `String` representing the ID of the exchange to check.
    ///
    /// Returns:
    ///     Result<bool, PostgresDBError>: `true` if the exchange exists, `false` otherwise.
    ///
    /// Raises:
    ///     PostgresDBError: If the check operation fails due to database errors.
    ///
    /// This method is asynchronous.
    pub async fn check_if_exchange_id_exists(
        &self,
        exchange_id_str: String,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_exchange_id_exists");
        let conn = &mut self.get_connection();

        match Exchange::check_if_exchange_id_exists(conn, exchange_id_str) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
        }
    }

    /// Reads an exchange from the database by its ID.
    ///
    /// Args:
    ///     exchange_id_str (String): A `String` representing the ID of the exchange to read.
    ///
    /// Returns:
    ///     Result<MetaExchange, PostgresDBError>: The `MetaExchange` object on success.
    ///
    /// Raises:
    ///     PostgresDBError: If the read operation fails due to database errors.
    ///
    /// This method is asynchronous.
    pub async fn read_exchange(
        &self,
        exchange_id_str: String,
    ) -> Result<MetaExchange, PostgresDBError> {
        self.dbg_print("read_exchange");
        let conn = &mut self.get_connection();

        match Exchange::read(conn, exchange_id_str) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Reads all exchanges from the database.
    ///
    /// Returns:
    ///     `Result<Vec<MetaExchange>, PostgresDBError>`
    ///
    /// A vector of `MetaExchange` objects on success.
    ///
    /// Raises:
    ///     PostgresDBError: If the read operation fails due to database errors.
    ///
    /// This method is asynchronous.
    pub async fn read_all_exchanges(&self) -> Result<Vec<MetaExchange>, PostgresDBError> {
        self.dbg_print("read_all_exchanges");
        let conn = &mut self.get_connection();

        match Exchange::read_all(conn) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Updates an existing exchange in the database.
    ///
    /// Args:
    ///     exchange_id_str (String): A `String` representing the ID of the exchange to update.
    ///     exchange (MetaExchange): A `MetaExchange` object containing the updated exchange data.
    ///
    /// Returns:
    ///     Result<usize, PostgresDBError>: The number of updated exchanges on success.
    ///
    /// Raises:
    ///     PostgresDBError: If the update operation fails due to database errors.
    ///
    /// This method is asynchronous.
    pub async fn update_exchange(
        &self,
        exchange_id_str: String,
        exchange: MetaExchange,
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("update_exchange");
        let conn = &mut self.get_connection();

        match Exchange::update(conn, exchange_id_str, exchange) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::UpdateFailed(e.to_string())),
        }
    }

    /// Deletes an exchange from the database by its ID.
    ///
    /// Args:
    ///     exchange_id_str (String): A `String` representing the ID of the exchange to delete.
    ///
    /// Returns:
    ///     Result<usize, PostgresDBError>: The number of deleted exchanges on success.
    ///
    /// Raises:
    ///     PostgresDBError: If the delete operation fails due to database errors.
    ///
    /// This method is asynchronous.
    pub async fn delete_exchange(&self, exchange_id_str: String) -> Result<usize, PostgresDBError> {
        self.dbg_print("delete_exchange");
        let conn = &mut self.get_connection();

        match Exchange::delete(conn, exchange_id_str) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::DeleteFailed(e.to_string())),
        }
    }
}
