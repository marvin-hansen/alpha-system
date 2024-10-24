use crate::model::instrument::{Instrument, UpdateInstrument};
use crate::prelude::CreateInstrumentsExchanges;
use crate::schema::mddb::instruments::table as instruments_table;
use crate::schema::mddb::instruments_exchanges::dsl::instruments_exchanges;
use crate::Connection;
use common_metadata::prelude::MetaInstrument;
use diesel::result::Error::DatabaseError;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl};

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
    /// A `Result` containing the inserted `MetaInstrument` if successful, else an `Error`.
    ///
    pub fn create_instrument(
        conn: &mut Connection,
        meta_instrument: MetaInstrument,
    ) -> Result<MetaInstrument, diesel::result::Error> {
        let instrument = Instrument::from_meta_instrument(meta_instrument.clone());
        let res = match diesel::insert_into(instruments_table)
            .values(&instrument)
            .get_result::<Instrument>(conn)
        {
            Ok(res) => Ok(res.to_meta_instrument()),
            Err(e) => Err(e),
        };

        let instrument_exchange = CreateInstrumentsExchanges::from_instrument(instrument.clone());
        match diesel::insert_into(instruments_exchanges)
            .values(instrument_exchange)
            .execute(conn)
        {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        // return the inserted instrument
        res
    }

    /// Inserts a collection of instruments into the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `meta_instruments` - A vector of `MetaInstrument` instances to be inserted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success of the operation.
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

        let instruments: Vec<Instrument> = meta_instruments
            .iter()
            .map(|meta_instrument| Instrument::from_meta_instrument(meta_instrument.clone()))
            .collect();

        // Insert the instruments one by one to prevent exceeding the number of parameters.
        // https://github.com/diesel-rs/diesel/issues/2414
        for instrument in &instruments {
            // println!();
            // println!("Inserting instrument: {:?}", &instrument.instrument_id);

            match diesel::insert_into(instruments_table)
                .values(instrument)
                .execute(conn)
            {
                Ok(_) => {}
                Err(e) => return Err(e),
            }

            // Insert the instrument exchanges relationship
            let instrument_exchange =
                CreateInstrumentsExchanges::from_instrument(instrument.clone());

            // println!();
            // println!(
            //     "Inserting instrument exchanges relationship: {:?} ",
            //     &instrument_exchange
            // );

            match diesel::insert_into(instruments_exchanges)
                .values(instrument_exchange)
                .execute(conn)
            {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(instruments.len())
    }

    /// Counts the number of instruments in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// * `Result<u64, Error>` - The total count of instruments if successful, an error otherwise.
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
    /// * `QueryResult<bool>` - A result indicating whether the instrument exists (true) or not (false).
    ///
    pub fn check_if_instrument_id_exists(
        conn: &mut Connection,
        instrument_id: &str,
    ) -> QueryResult<bool> {
        let exists = instruments_table
            .find(instrument_id)
            .first::<Instrument>(conn)
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
    /// A `Result` containing the retrieved `MetaInstrument` if successful, else an `Error`.
    pub fn read(
        conn: &mut Connection,
        instrument_id: &str,
    ) -> Result<MetaInstrument, diesel::result::Error> {
        instruments_table
            .filter(crate::schema::mddb::instruments::instrument_id.eq(instrument_id))
            .first::<Instrument>(conn)
            .map(|instrument| instrument.to_meta_instrument())
    }

    /// Reads all instruments from the database and converts them to `MetaInstrument` objects.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `MetaInstrument` objects if successful, else a `diesel::result::Error`.
    pub fn read_all(conn: &mut Connection) -> Result<Vec<MetaInstrument>, diesel::result::Error> {
        instruments_table
            .load::<Instrument>(conn)
            .map(|instruments| {
                instruments
                    .iter()
                    .map(|instrument| instrument.to_meta_instrument())
                    .collect()
            })
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
    /// A `Result` indicating the number of rows affected if successful, else an `Error`.
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
    /// Returns a `Result` containing the number of rows affected by the delete operation.
    /// If the instrument does not exist, the query will return `Ok(0)`.
    /// If the instrument exists and was deleted, the query will return `Ok(1)`.
    ///
    /// Note, delete only returns an error when either the database connection or the query fails.
    pub fn delete(
        conn: &mut Connection,
        param_instrument_id: String,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::mddb::instruments_exchanges::dsl::instruments_exchanges;
        use crate::schema::mddb::instruments_exchanges::{exchange_id, instrument_id};

        // Check if the instrument exists
        let exists = Instrument::check_if_instrument_id_exists(conn, &param_instrument_id)
            .expect("Failed to check if instrument ID exists");

        // Return Ok(0) if the instrument does not exist
        if !exists {
            return Ok(0);
        }

        // Read the instrument before deleting it so we can get the exchange_id
        let instrument =
            Instrument::read(conn, &param_instrument_id).expect("Failed to read instrument");
        let param_exchange_id = instrument.exchange_code;

        // Delete the instrument exchange relation
        diesel::delete(
            instruments_exchanges
                .filter(instrument_id.eq(param_instrument_id.clone()))
                .filter(exchange_id.eq(param_exchange_id)),
        )
        .execute(conn)
        .expect("Failed to delete instrument exchange relation");

        // Delete the instrument
        let res = diesel::delete(
            instruments_table
                .filter(crate::schema::mddb::instruments::instrument_id.eq(&param_instrument_id)),
        )
        .execute(conn);

        // Return the QueryResult containing the number of deleted rows
        res
    }
}
