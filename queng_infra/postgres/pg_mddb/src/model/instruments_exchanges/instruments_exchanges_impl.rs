use crate::model::instruments_exchanges::{CreateInstrumentsExchanges, InstrumentsExchanges};
use crate::schema::mddb::instruments_exchanges::dsl::instruments_exchanges;
use crate::schema::mddb::instruments_exchanges::{exchange_id, instrument_id};
use crate::Connection;
use common_metadata::prelude::MetaInstrument;
use diesel::dsl::insert_into;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

impl InstrumentsExchanges {
    /// Creates a new entry in the InstrumentsExchanges table with the provided instrument and exchange IDs.
    ///
    /// # Arguments
    /// - `conn`: A mutable reference to the database connection.
    /// - `param_instrument_id`: The ID of the instrument to associate.
    /// - `param_exchange_id`: The ID of the exchange to associate.
    ///
    /// # Returns
    /// A `QueryResult` containing the newly created InstrumentsExchanges entry if successful.
    ///
    pub fn create_instruments_exchange(
        conn: &mut Connection,
        param_instrument_id: String,
        param_exchange_id: String,
    ) -> QueryResult<Self> {
        let new_entry = CreateInstrumentsExchanges {
            instrument_id: param_instrument_id,
            exchange_id: param_exchange_id,
        };

        insert_into(instruments_exchanges)
            .values(&new_entry)
            .returning(InstrumentsExchanges::as_returning())
            .get_result::<Self>(conn)
    }

    pub fn create_instruments_exchange_collection(
        conn: &mut Connection,
        meta_instruments: &[MetaInstrument],
    ) -> Result<usize, diesel::result::Error> {
        let instruments: Vec<CreateInstrumentsExchanges> = meta_instruments
            .iter()
            .map(|meta_instrument| {
                CreateInstrumentsExchanges::from_meta_instrument(meta_instrument.clone())
            })
            .collect();

        // Insert the instruments one by one to prevent exceeding the number of parameters.
        // https://github.com/diesel-rs/diesel/issues/2414
        for i in instruments.iter() {
            if let Err(e) = diesel::insert_into(instruments_exchanges)
                .values(i)
                .execute(conn)
            {
                return Err(e);
            }
        }

        Ok(instruments.len())
    }

    /// Checks if an entry with the provided instrument ID and exchange ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `param_instrument_id` - The ID of the instrument to check.
    /// * `param_exchange_id` - The ID of the exchange to check.
    ///
    /// # Returns
    ///
    /// * `QueryResult<bool>` - A result indicating whether the entry exists (true) or not (false).
    ///
    pub fn check_if_exists(
        conn: &mut Connection,
        param_instrument_id: String,
        param_exchange_id: String,
    ) -> QueryResult<bool> {
        match instruments_exchanges
            .find((param_instrument_id, param_exchange_id))
            .first::<Self>(conn)
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Deletes an entry from the instruments_exchanges table based on the provided instrument ID and exchange ID.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `param_instrument_id` - The ID of the instrument to be deleted.
    /// * `param_exchange_id` - The ID of the exchange to be deleted.
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing the number of rows affected by the delete operation.
    ///
    pub fn delete(
        conn: &mut Connection,
        param_instrument_id: String,
        param_exchange_id: String,
    ) -> QueryResult<usize> {
        diesel::delete(
            instruments_exchanges
                .filter(instrument_id.eq(&param_instrument_id))
                .filter(exchange_id.eq(&param_exchange_id)),
        )
        .execute(conn)
    }
}
