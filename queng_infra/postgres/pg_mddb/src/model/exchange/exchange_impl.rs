/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::Connection;
use crate::model::exchange::{CreateExchange, Exchange, UpdateExchange};
use crate::schema::mddb::exchanges::dsl::exchanges as exchanges_table;
use crate::schema::mddb::exchanges::exchange_id;
use common_metadata::MetaExchange;
use diesel::result::Error;
use diesel::result::Error::DatabaseError;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

impl Exchange {
    /// Creates a new exchange in the database based on the provided `MetaExchange` information.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `meta_exchange` - The `MetaExchange` struct containing the exchange information to be inserted.
    ///
    /// # Returns
    /// * `Result<MetaExchange, Error>` - The created exchange if successful
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Unique constraint violations (if `exchange_id` already exists)
    /// * Invalid data in `meta_exchange` (constraint violations)
    /// * Serialization errors when converting between types
    ///
    pub fn create(
        conn: &mut Connection,
        meta_exchange: MetaExchange,
    ) -> Result<MetaExchange, Error> {
        let new_exchange = CreateExchange::from_meta_exchange(meta_exchange);
        match diesel::insert_into(exchanges_table)
            .values(&new_exchange)
            .get_result::<Self>(conn)
        {
            Ok(res) => Ok(res.to_meta_exchange()),
            Err(e) => Err(e),
        }
    }

    /// Creates a collection of exchanges in the database based on the provided metadata exchanges.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `meta_exchanges` - A vector of `MetaExchange` structs containing metadata for the exchanges to be created.
    ///
    /// # Returns
    ///
    /// * `Result<usize, Error>` - Number of exchanges successfully created
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Empty collection provided (`DatabaseError(Unknown)`)
    /// * Database connection errors
    /// * Unique constraint violations
    /// * Data validation errors for any exchange in the collection
    /// * Batch insertion failures
    /// * Transaction rollback if any exchange fails to insert
    ///
    pub fn create_collection(
        conn: &mut Connection,
        meta_exchanges: &[MetaExchange],
    ) -> Result<usize, Error> {
        if meta_exchanges.is_empty() {
            return Err(DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(String::from(
                    "[create_exchange_collection] No exchanges provided. Collection is empty.",
                )),
            ));
        }

        let new_exchanges: Vec<CreateExchange> = meta_exchanges
            .iter()
            .map(|me| CreateExchange::from_meta_exchange(me.clone()))
            .collect();

        match diesel::insert_into(exchanges_table)
            .values(&new_exchanges)
            .execute(conn)
        {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    /// Retrieves the total count of exchanges from the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// * `Result<u64, Error>` - Total count of exchanges
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Type conversion errors when converting count from i64 to u64
    ///
    pub fn count(conn: &mut Connection) -> Result<u64, Error> {
        exchanges_table
            .count()
            .get_result::<i64>(conn)
            .map(|c| c as u64)
    }

    /// Checks if an exchange with the given exchange ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `exchange_id_str` - The ID of the exchange to check for existence.
    ///
    /// # Returns
    ///
    /// * `Result<bool, Error>` - `true` if the exchange exists, `false` otherwise
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Note: Not finding the exchange is NOT an error, it returns `Ok(false)`
    ///
    pub fn check_if_exchange_id_exists(
        conn: &mut Connection,
        exchange_id_str: String,
    ) -> Result<bool, Error> {
        let exists = diesel::select(diesel::dsl::exists(
            exchanges_table.filter(exchange_id.eq(exchange_id_str)),
        ))
        .get_result(conn)?;
        Ok(exists)
    }

    /// Reads a `MetaExchange` from the database based on the provided exchange ID string.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `param_exchange_id` - A String representing the exchange ID to search for.
    ///
    /// # Returns
    ///
    /// * `Result<Option<MetaExchange>, Error>` - The exchange if found, None if not found
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Data deserialization errors
    /// * Note: Not finding the exchange is NOT an error, it returns `Ok(None)`
    ///
    pub fn read(
        conn: &mut Connection,
        param_exchange_id: String,
    ) -> Result<Option<MetaExchange>, Error> {
        let exists = Self::check_if_exchange_id_exists(conn, param_exchange_id.clone())?;
        if !exists {
            Ok(None)
        } else {
            exchanges_table
                .filter(exchange_id.eq(param_exchange_id))
                .first::<Self>(conn)
                .map(|e| Some(e.to_meta_exchange()))
        }
    }

    /// Reads all exchanges from the database and converts them to `MetaExchange` objects.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<MetaExchange>, Error>` - Vector of all exchanges
    ///   Returns an empty vector if no exchanges exist
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Data deserialization errors when converting to `MetaExchange`
    /// * Memory allocation errors for large result sets
    ///
    pub fn read_all(conn: &mut Connection) -> Result<Vec<MetaExchange>, Error> {
        exchanges_table.load::<Self>(conn).map(|e| {
            e.into_iter()
                .map(|exchange| exchange.to_meta_exchange())
                .collect()
        })
    }

    /// Updates an exchange in the database based on the provided exchange ID.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `exchange_id_str` - The ID of the exchange to update.
    /// * `meta_exchange` - The updated exchange metadata.
    ///
    /// # Returns
    ///
    /// * `Result<usize, Error>` - Number of rows affected (0 if not found, 1 if updated)
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Constraint violations in the updated data
    /// * Data validation errors
    /// * Note: Not finding the exchange is NOT an error, it returns `Ok(0)`
    ///
    pub fn update(
        conn: &mut Connection,
        exchange_id_str: String,
        meta_exchange: MetaExchange,
    ) -> Result<usize, Error> {
        let updated_exchange = UpdateExchange::from_meta_exchange(meta_exchange);
        diesel::update(exchanges_table.filter(exchange_id.eq(exchange_id_str)))
            .set(&updated_exchange)
            .execute(conn)
    }

    /// Deletes an exchange from the database by exchange ID.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `exchange_id_str` - The ID of the exchange to delete.
    ///
    /// # Returns
    ///
    /// * `Result<usize, Error>` - Number of rows affected:
    ///   - Returns `Ok(0)` if the exchange didn't exist
    ///   - Returns `Ok(1)` if the exchange was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Foreign key constraint violations (if exchange is referenced elsewhere)
    /// * Transaction failures during deletion
    /// * Concurrent modification conflicts
    ///
    pub fn delete(conn: &mut Connection, exchange_id_str: String) -> Result<usize, Error> {
        diesel::delete(exchanges_table.filter(exchange_id.eq(exchange_id_str))).execute(conn)
    }
}
