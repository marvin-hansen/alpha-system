/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::PostgresMDDBManager;
use common_errors::PostgresDBError;
use common_metadata::MetaInstrument;
use pg_mddb::Instrument;

impl PostgresMDDBManager {
    /// Inserts a new instrument into the database.
    ///
    /// Args:
    ///     instrument (MetaInstrument): The instrument to be inserted.
    ///
    /// Returns:
    ///     Result<`MetaInstrument`, PostgresDBError>: The inserted instrument on success, or a `PostgresDBError` on failure.
    ///
    /// Raises:
    ///     - `PostgresDBError::InsertFailed`: If the insertion operation fails due to database errors.
    pub async fn insert_instrument(
        &self,
        instrument: MetaInstrument,
    ) -> Result<MetaInstrument, PostgresDBError> {
        self.dbg_print("insert_instruments");
        let conn = &mut self.get_connection();

        match Instrument::create_instrument(conn, instrument) {
            Ok(instrument) => Ok(instrument),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    /// Inserts a collection of instruments into the database.
    ///
    /// Args:
    ///     instruments (&[`MetaInstrument`]): A slice of instruments to be inserted.
    ///
    /// Returns:
    ///     Result<usize, PostgresDBError>: The number of instruments successfully inserted, or a `PostgresDBError` on failure.
    ///
    /// Raises:
    ///     - `PostgresDBError::InsertFailed`: If the insertion operation fails due to database errors.
    pub async fn insert_instrument_collection(
        &self,
        instruments: &[MetaInstrument],
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("insert_instruments_collection");
        let conn = &mut self.get_connection();

        match Instrument::create_instrument_collection(conn, instruments) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    /// Counts the total number of instruments in the database.
    ///
    /// Returns:
    ///     `Result<u64, PostgresDBError>`: The count of instruments on success, or a `PostgresDBError` on failure.
    ///
    /// Raises:
    ///     - `PostgresDBError::CountFailed`: If the count operation fails due to database errors.
    pub async fn count_instruments(&self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_instruments");
        let conn = &mut self.get_connection();

        match Instrument::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
        }
    }

    /// Checks if an instrument with the specified ID exists in the database.
    ///
    /// Args:
    ///     `instrument_id` (&str): The ID of the instrument to check.
    ///
    /// Returns:
    ///     `Result<bool, PostgresDBError>`: `true` if the instrument exists, `false` otherwise, or a `PostgresDBError` on failure.
    ///
    /// Raises:
    ///     - `PostgresDBError::CheckIfExistsFailed`: If the check operation fails due to database errors.
    pub async fn check_if_instrument_id_exists(
        &self,
        instrument_id: &str,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_instrument_id_exists");
        let conn = &mut self.get_connection();

        match Instrument::check_if_instrument_id_exists(conn, instrument_id) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        }
    }

    /// Reads an instrument from the database by its ID.
    ///
    /// Args:
    ///     `instrument_id` (&str): The ID of the instrument to read.
    ///
    /// Returns:
    ///     `Result<MetaInstrument, PostgresDBError>`: The instrument on success, or a `PostgresDBError` on failure.
    ///
    /// Raises:
    ///     - `PostgresDBError::QueryFailed`: If the read operation fails due to database errors.
    pub async fn read_instrument(
        &self,
        instrument_id: &str,
    ) -> Result<Option<MetaInstrument>, PostgresDBError> {
        self.dbg_print("read_instrument");
        let conn = &mut self.get_connection();

        match Instrument::read(conn, instrument_id) {
            Ok(instrument) => Ok(instrument),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Reads all instruments from the database.
    ///
    /// Returns:
    ///     `Result<Vec<MetaInstrument>, PostgresDBError>`: A vector of all instruments on success, or a `PostgresDBError` on failure.
    ///
    /// Raises:
    ///     - `PostgresDBError::QueryFailed`: If the read operation fails due to database errors.
    pub async fn read_all_instruments(&self) -> Result<Vec<MetaInstrument>, PostgresDBError> {
        self.dbg_print("read_all_instruments");
        let conn = &mut self.get_connection();

        match Instrument::read_all(conn) {
            Ok(instruments) => Ok(instruments),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Updates an existing instrument in the database.
    ///
    /// Args:
    ///     `instrument_id` (&str): The ID of the instrument to update.
    ///     instrument (MetaInstrument): The new instrument data.
    ///
    /// Returns:
    ///     `Result<usize, PostgresDBError>`: The number of rows affected on success, or a `PostgresDBError` on failure.
    ///
    /// Raises:
    ///     - `PostgresDBError::UpdateFailed`: If the update operation fails due to database errors.
    pub async fn update_instrument(
        &self,
        instrument_id: &str,
        instrument: MetaInstrument,
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("update_instrument");
        let conn = &mut self.get_connection();

        match Instrument::update(conn, instrument_id, instrument) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::UpdateFailed(e.to_string())),
        }
    }

    /// Deletes an instrument from the database by its ID.
    ///
    /// Args:
    ///     `instrument_id` (String): The ID of the instrument to delete.
    ///
    /// Returns:
    ///     `Result<usize, PostgresDBError>`: The number of rows affected on success, or a `PostgresDBError` on failure.
    ///
    /// Raises:
    ///     - `PostgresDBError::DeleteFailed`: If the delete operation fails due to database errors.
    pub async fn delete_instrument(&self, instrument_id: String) -> Result<usize, PostgresDBError> {
        self.dbg_print("delete_instrument");
        let conn = &mut self.get_connection();

        match Instrument::delete(conn, instrument_id) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::DeleteFailed(e.to_string())),
        }
    }
}
