use crate::model::instrument::{CreateInstrument, Instrument, UpdateInstrument};
use crate::schema::cmdb::instrument::dsl::*;
use crate::Connection;
use common_exchange::prelude::Instrument as CommonInstrument;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

impl Instrument {
    /// Creates a new instrument in the database and returns it as a `CommonInstrument`.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `item` - The `CreateInstrument` struct containing the instrument data.
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing the created `CommonInstrument`.
    ///
    /// # Errors
    ///
    /// If there is an error with the database connection or the insert fails,
    /// this function will return an `Err` containing the error.
    ///
    pub fn create(db: &mut Connection, ins: &CommonInstrument) -> QueryResult<CommonInstrument> {
        let item = CreateInstrument::from_common_instrument(ins);
        diesel::insert_into(instrument)
            .values(item)
            .returning(Instrument::as_returning())
            .get_result::<Instrument>(db)
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
    /// If there is an error with the database connection, this function will return an `Err` containing the error.
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
    /// `Result<bool, diesel::result::Error>` - A result indicating success or failure.
    /// * If the instrument exists, returns `Ok(true)`,
    /// * otherwise `Ok(false)`.
    ///
    pub fn check_if_instrument_code_exists(
        db: &mut Connection,
        param_instrument_code: String,
    ) -> QueryResult<bool> {
        match instrument
            .find(param_instrument_code)
            .first::<Instrument>(db)
        {
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
    /// A `QueryResult` containing the instrument as a `CommonInstrument`.
    ///
    pub fn read(db: &mut Connection, param_code: String) -> QueryResult<CommonInstrument> {
        instrument
            .filter(code.eq(param_code))
            .limit(1)
            .get_result::<Instrument>(db)
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
    /// A `QueryResult` containing the vector of `CommonInstrument`s.
    ///
    pub fn read_all(db: &mut Connection) -> QueryResult<Vec<CommonInstrument>> {
        instrument
            .load::<Instrument>(db)
            .map(|s| s.iter().map(|s| s.to_common_instrument()).collect())
    }

    /// Updates an instrument's record in the database
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection
    /// * `param_code` - The instrument code to update
    /// * `item` - The new instrument data
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult` containing the updated instrument as a `CommonInstrument`
    ///
    pub fn update(
        db: &mut Connection,
        param_code: String,
        ins: &CommonInstrument,
    ) -> QueryResult<CommonInstrument> {
        let item = UpdateInstrument::from_common_instrument(ins);

        diesel::update(instrument.filter(code.eq(param_code)))
            .set(item)
            .returning(Instrument::as_returning())
            .get_result::<Instrument>(db)
            .map(|s| s.to_common_instrument())
    }

    /// Deletes a `Instrument` by its `code`.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection.
    /// * `param_code` - The `code` of the `Instrument` to be deleted.
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing the number of rows deleted.
    ///
    /// # Errors
    ///
    /// If there is an error with the database connection, this function will return
    /// an `Err` containing the error.
    ///
    pub fn delete(db: &mut Connection, param_code: String) -> QueryResult<usize> {
        diesel::delete(instrument.filter(code.eq(param_code))).execute(db)
    }
}
