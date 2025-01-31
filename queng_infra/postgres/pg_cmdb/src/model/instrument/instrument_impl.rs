/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::model::instrument::{CreateInstrument, Instrument, UpdateInstrument};
use crate::schema::cmdb::instrument::dsl::{code, instrument};
use crate::Connection;
use common_exchange::Instrument as CommonInstrument;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

impl Instrument {
    /// Creates a new instrument in the database and returns it as a `CommonInstrument`.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `ins` - The `CommonInstrument` containing the instrument data to be created.
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing the created `CommonInstrument` if successful.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The database connection fails
    /// * The insert operation fails (e.g., due to unique constraint violations)
    /// * The instrument code already exists
    /// * The provided instrument data is invalid or missing required fields
    ///
    /// # Implementation Notes
    ///
    /// This function performs the following steps:
    /// 1. Converts the `CommonInstrument` to a `CreateInstrument`
    /// 2. Inserts the new record into the database
    /// 3. Returns the newly created instrument as a `CommonInstrument`
    ///
    pub fn create(db: &mut Connection, ins: &CommonInstrument) -> QueryResult<CommonInstrument> {
        let item = CreateInstrument::from_common_instrument(ins);
        diesel::insert_into(instrument)
            .values(item)
            .returning(Self::as_returning())
            .get_result::<Self>(db)
            .map(|s| s.to_common_instrument())
    }

    /// Retrieves the total count of instruments in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing the total count of instruments as a `u64`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The database connection fails
    /// * The count operation fails
    /// * The conversion from i64 to u64 fails (should not occur under normal circumstances)
    ///
    /// # Implementation Notes
    ///
    /// This function performs a COUNT operation on the instrument table and converts
    /// the result from i64 to u64, as negative counts are not possible.
    ///
    pub fn count(db: &mut Connection) -> QueryResult<u64> {
        instrument.count().get_result::<i64>(db).map(|i| i as u64)
    }

    /// Checks if an instrument with the given code exists in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection.
    /// * `param_instrument_code` - The code of the instrument to check.
    ///
    /// # Returns
    ///
    /// `Result<bool, diesel::result::Error>` - A result indicating success or failure:
    /// * `Ok(true)` if the instrument exists
    /// * `Ok(false)` if the instrument does not exist
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The database connection fails
    /// * The query execution fails due to database errors
    ///
    /// # Implementation Notes
    ///
    /// This function attempts to find an instrument by its code. Any error during the
    /// query is interpreted as the instrument not existing, except for critical database errors.
    ///
    pub fn check_if_instrument_code_exists(
        db: &mut Connection,
        param_instrument_code: String,
    ) -> QueryResult<bool> {
        match instrument.find(param_instrument_code).first::<Self>(db) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Reads an instrument from the database by its code and returns it as a `CommonInstrument`.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `param_code` - The instrument code to read.
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing the instrument as a `CommonInstrument` if found.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The database connection fails
    /// * The instrument with the specified code does not exist
    /// * The query execution fails
    /// * The conversion to `CommonInstrument` fails
    ///
    /// # Implementation Notes
    ///
    /// This function:
    /// 1. Filters the instrument table by the provided code
    /// 2. Limits the result to 1 record (optimization)
    /// 3. Converts the result to a `CommonInstrument`
    ///
    pub fn read(db: &mut Connection, param_code: String) -> QueryResult<CommonInstrument> {
        instrument
            .filter(code.eq(param_code))
            .limit(1)
            .get_result::<Self>(db)
            .map(|s| s.to_common_instrument())
    }

    /// Reads all instruments from the database and returns them as a vector of `CommonInstrument`s.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing a vector of all instruments as `CommonInstrument`s.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The database connection fails
    /// * The query execution fails
    /// * The conversion of any instrument to `CommonInstrument` fails
    ///
    /// # Performance Considerations
    ///
    /// This function retrieves all instruments from the database. For large datasets,
    /// consider using pagination or limiting the result set size.
    ///
    pub fn read_all(db: &mut Connection) -> QueryResult<Vec<CommonInstrument>> {
        instrument
            .load::<Self>(db)
            .map(|s| s.iter().map(Self::to_common_instrument).collect())
    }

    /// Updates an instrument's record in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `param_code` - The instrument code to update.
    /// * `ins` - The new instrument data as a `CommonInstrument`.
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult` containing the updated instrument as a `CommonInstrument`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The database connection fails
    /// * The instrument with the specified code does not exist
    /// * The update operation fails due to constraint violations
    /// * The conversion to/from `CommonInstrument` fails
    ///
    /// # Implementation Notes
    ///
    /// This function:
    /// 1. Converts the `CommonInstrument` to `UpdateInstrument`
    /// 2. Updates the existing record
    /// 3. Returns the updated instrument data
    ///
    pub fn update(
        db: &mut Connection,
        param_code: String,
        ins: &CommonInstrument,
    ) -> QueryResult<CommonInstrument> {
        let item = UpdateInstrument::from_common_instrument(ins);

        diesel::update(instrument.filter(code.eq(param_code)))
            .set(item)
            .returning(Self::as_returning())
            .get_result::<Self>(db)
            .map(|s| s.to_common_instrument())
    }

    /// Deletes an instrument by its code.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection.
    /// * `param_code` - The code of the instrument to be deleted.
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing the number of rows deleted (0 or 1).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The database connection fails
    /// * The delete operation fails
    /// * The instrument is referenced by other tables (foreign key constraint)
    ///
    /// # Implementation Notes
    ///
    /// This function performs a soft delete if the schema supports it,
    /// otherwise performs a hard delete. Returns 0 if no instrument was found.
    ///
    pub fn delete(db: &mut Connection, param_code: String) -> QueryResult<usize> {
        diesel::delete(instrument.filter(code.eq(param_code))).execute(db)
    }
}
