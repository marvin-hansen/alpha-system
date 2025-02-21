/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::Connection;
use crate::model::instrument::{Instrument, UpdateInstrument};
use crate::schema::mddb::instruments::table as instruments_table;
use common_metadata::MetaInstrument;
use diesel::result::Error::DatabaseError;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl};

const MAX_PARAMETERS: usize = 65000;
const MAX_BATCH: usize = 5000;

impl Instrument {
    /// Creates a new instrument entry in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `meta_instrument` - The metadata of the instrument to be inserted.
    ///
    /// # Returns
    ///
    /// * `Result<MetaInstrument, diesel::result::Error>` - The created instrument if successful
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Unique constraint violations (if `instrument_id` already exists)
    /// * Invalid data in `meta_instrument` (constraint violations)
    /// * Serialization errors when converting between types
    ///
    pub fn create_instrument(
        conn: &mut Connection,
        meta_instrument: MetaInstrument,
    ) -> Result<MetaInstrument, diesel::result::Error> {
        let instrument = Self::from_meta_instrument(meta_instrument);
        match diesel::insert_into(instruments_table)
            .values(&instrument)
            .get_result::<Self>(conn)
        {
            Ok(res) => Ok(res.to_meta_instrument()),
            Err(e) => Err(e),
        }
    }

    /// Batch inserts a collection of instruments into the database.
    ///
    /// For large collections (> `MAX_BATCH`), instruments are inserted in chunks
    /// to prevent exceeding database parameter limits.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `meta_instruments` - A vector of `MetaInstrument` instances to be inserted.
    ///
    /// # Returns
    ///
    /// * `Result<usize, diesel::result::Error>` - Number of instruments successfully inserted
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Empty collection provided (`DatabaseError(Unknown)`)
    /// * Database connection errors
    /// * Unique constraint violations
    /// * Batch processing failures
    /// * Data validation errors for any instrument in the collection
    /// * Transaction failures during chunk processing
    ///
    pub fn create_instrument_collection(
        conn: &mut Connection,
        meta_instruments: &[MetaInstrument],
    ) -> Result<usize, diesel::result::Error> {
        if meta_instruments.is_empty() {
            return Err(DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(String::from(
                    "[create_instrument_collection]: No instruments provided. Collection is empty.",
                )),
            ));
        }

        // Convert the meta_instruments to instruments
        let instruments: Vec<Self> = meta_instruments
            .iter()
            .map(|meta_instrument| Self::from_meta_instrument(meta_instrument.clone()))
            .collect();

        let number_instruments = instruments.len();
        if number_instruments > MAX_BATCH {
            // Insert the instruments in chunks to prevent exceeding the number of parameters.
            // https://github.com/diesel-rs/diesel/issues/2414
            let chunk_size = MAX_PARAMETERS.div_ceil(number_instruments) + 10; // Division without remainder plus one
            for chunk in instruments.chunks(chunk_size) {
                match diesel::insert_into(instruments_table)
                    .values(chunk)
                    .execute(conn)
                {
                    Ok(_) => {} // Do nothing in case of successful chunk insert
                    Err(e) => return Err(e),
                };
            }
        } else {
            match diesel::insert_into(instruments_table)
                .values(&instruments)
                .execute(conn)
            {
                Ok(_) => {} // Do nothing in case of successful insert
                Err(e) => return Err(e),
            };
        }

        Ok(number_instruments)
    }

    /// Counts the number of instruments in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// * `Result<u64, diesel::result::Error>` - Total count of instruments
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Type conversion errors when converting count from i64 to u64
    ///
    pub fn count(conn: &mut Connection) -> Result<u64, diesel::result::Error> {
        instruments_table
            .count()
            .get_result::<i64>(conn)
            .map(|c| c as u64)
    }

    /// Checks if an instrument with the given ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `instrument_id` - The ID of the instrument to check.
    ///
    /// # Returns
    ///
    /// * `QueryResult<bool>` - `true` if the instrument exists, `false` otherwise
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Note: Not finding the instrument is NOT an error, it returns `Ok(false)`
    ///
    pub fn check_if_instrument_id_exists(
        conn: &mut Connection,
        instrument_id: &str,
    ) -> QueryResult<bool> {
        let exists = instruments_table
            .find(instrument_id)
            .first::<Self>(conn)
            .optional()?
            .is_some();
        Ok(exists)
    }

    /// Reads a `MetaInstrument` from the database based on the provided instrument ID.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `instrument_id` - The ID of the instrument to read.
    ///
    /// # Returns
    ///
    /// * `Result<Option<MetaInstrument>, diesel::result::Error>` - The instrument if found, None if not found
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Data deserialization errors
    /// * Note: Not finding the instrument is NOT an error, it returns `Ok(None)`
    ///
    pub fn read(
        conn: &mut Connection,
        instrument_id: &str,
    ) -> Result<Option<MetaInstrument>, diesel::result::Error> {
        let exists = Self::check_if_instrument_id_exists(conn, instrument_id)?;
        if !exists {
            Ok(None)
        } else {
            instruments_table
                .filter(crate::schema::mddb::instruments::instrument_id.eq(instrument_id))
                .first::<Self>(conn)
                .map(|instrument| Some(instrument.to_meta_instrument()))
        }
    }

    /// Reads all instruments from the database and converts them to `MetaInstrument` objects.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<MetaInstrument>, diesel::result::Error>` - Vector of all instruments
    ///   Returns an empty vector if no instruments exist
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Data deserialization errors when converting to `MetaInstrument`
    /// * Memory allocation errors for large result sets
    ///
    pub fn read_all(conn: &mut Connection) -> Result<Vec<MetaInstrument>, diesel::result::Error> {
        instruments_table
            .load::<Self>(conn)
            .map(|instruments| instruments.iter().map(Self::to_meta_instrument).collect())
    }

    /// Updates an instrument entry in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `instrument_id` - The ID of the instrument to be updated.
    /// * `meta_instrument` - The updated metadata of the instrument.
    ///
    /// # Returns
    ///
    /// * `Result<usize, diesel::result::Error>` - Number of rows affected (0 if not found, 1 if updated)
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Constraint violations in the updated data
    /// * Data validation errors
    /// * Note: Not finding the instrument is NOT an error, it returns `Ok(0)`
    ///
    pub fn update(
        conn: &mut Connection,
        instrument_id: &str,
        meta_instrument: MetaInstrument,
    ) -> Result<usize, diesel::result::Error> {
        let update_instrument = UpdateInstrument::from_meta_instrument(meta_instrument);
        diesel::update(
            instruments_table
                .filter(crate::schema::mddb::instruments::instrument_id.eq(instrument_id)),
        )
        .set(&update_instrument)
        .execute(conn)
    }

    /// Deletes an instrument from the database by instrument ID.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `instrument_id` - The ID of the instrument to delete.
    ///
    /// # Returns
    ///
    /// * `Result<usize, diesel::result::Error>` - Number of rows affected:
    ///   - Returns `Ok(0)` if the instrument didn't exist
    ///   - Returns `Ok(1)` if the instrument was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Foreign key constraint violations (if instrument is referenced elsewhere)
    /// * Transaction failures during deletion
    /// * Panics if `check_if_instrument_id_exists` fails
    ///
    pub fn delete(
        conn: &mut Connection,
        param_instrument_id: String,
    ) -> Result<usize, diesel::result::Error> {
        // Check if the instrument exists
        let exists = Self::check_if_instrument_id_exists(conn, &param_instrument_id)
            .expect("Failed to check if instrument ID exists");

        // Return Ok(0) if the instrument does not exist
        if !exists {
            return Ok(0);
        }

        // Return the QueryResult containing the number of deleted rows
        diesel::delete(
            instruments_table
                .filter(crate::schema::mddb::instruments::instrument_id.eq(&param_instrument_id)),
        )
        .execute(conn)
    }
}
