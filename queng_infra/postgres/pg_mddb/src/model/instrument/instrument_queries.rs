use crate::prelude::Instrument;
use crate::schema::mddb::instruments::{
    instrument_base_asset, instrument_code, instrument_exchanges_code, instrument_quote_asset,
    table as instruments_table,
};
use crate::Connection;
use common_metadata::prelude::MetaInstrument;
use diesel::associations::HasTable;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};

impl Instrument {
    /// Queries the database for an instrument based on the provided instrument code.
    ///
    /// # Arguments
    /// - `conn`: A mutable reference to the database connection.
    /// - `param_instrument_code`: A string slice representing the instrument code to search for.
    ///
    /// # Returns
    /// - `Result<Option<MetaInstrument>, diesel::result::Error>`
    ///
    /// A Result containing either Some(MetaInstrument) if the instrument is found, or None if not found.

    pub fn query_instruments_by_code(
        conn: &mut Connection,
        param_instrument_code: &str,
    ) -> Result<Option<MetaInstrument>, diesel::result::Error> {
        match instruments_table::table()
            .filter(instrument_code.eq(param_instrument_code))
            .first::<Instrument>(conn)
        {
            Ok(res) => Ok(Some(res.to_meta_instrument())),
            Err(_) => Ok(None),
        }
    }

    /// Queries instruments by FIGI from the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `param_instrument_pair_figi` - The FIGI of the instrument pair to query.
    ///
    /// # Returns
    ///
    /// * `Result<Option<MetaInstrument>, diesel::result::Error>`
    ///
    /// An optional `MetaInstrument` if found, else `None`.
    ///
    pub fn query_instruments_by_figi(
        conn: &mut Connection,
        param_instrument_pair_figi: &str,
    ) -> Result<Option<MetaInstrument>, diesel::result::Error> {
        match instruments_table::table()
            .filter(instrument_base_asset.eq(param_instrument_pair_figi))
            .first::<Instrument>(conn)
        {
            Ok(res) => Ok(Some(res.to_meta_instrument())),
            Err(_) => Ok(None),
        }
    }

    /// Queries all instruments for a given exchange code from the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `param_exchange_code` - A string slice that holds the exchange code
    ///   for which instruments need to be queried.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `MetaInstrument` objects if successful,
    /// or a `diesel::result::Error` if an error occurs during the query.
    ///
    pub fn query_all_instruments_for_exchange(
        conn: &mut Connection,
        param_exchange_code: &str,
    ) -> Result<Vec<MetaInstrument>, diesel::result::Error> {
        match instruments_table::table()
            .filter(instrument_exchanges_code.eq(param_exchange_code))
            .load::<Instrument>(conn)
        {
            Ok(res) => {
                // Convert the Vec<Instrument> into Vec<MetaInstrument>
                Ok(res.iter().map(|i| i.to_meta_instrument()).collect())
            }
            Err(e) => Err(e),
        }
    }

    /// Queries all instruments for a specific base asset.
    ///
    /// # Arguments
    /// - `conn`: A mutable reference to the database connection.
    /// - `param_base_asset`: A string slice representing the base asset to filter the instruments by.
    ///
    /// # Returns
    /// A Result containing a vector of MetaInstrument if the query is successful, otherwise a diesel::result::Error.
    pub fn query_all_instruments_for_base_asset(
        conn: &mut Connection,
        param_base_asset: &str,
    ) -> Result<Vec<MetaInstrument>, diesel::result::Error> {
        match instruments_table::table()
            .filter(instrument_base_asset.eq(param_base_asset))
            .load::<Instrument>(conn)
        {
            Ok(res) => {
                // Convert the Vec<Instrument> into Vec<MetaInstrument>
                Ok(res.iter().map(|i| i.to_meta_instrument()).collect())
            }
            Err(e) => Err(e),
        }
    }

    /// Queries all instruments for a specific quote asset.
    ///
    /// # Arguments
    /// - `conn`: A mutable reference to the database connection.
    /// - `param_quote_asset`: A string slice representing the quote asset to filter instruments by.
    ///
    /// # Returns
    /// A Result containing a vector of MetaInstrument if the query is successful, otherwise a diesel::result::Error.
    pub fn query_all_instruments_for_quote_asset(
        conn: &mut Connection,
        param_quote_asset: &str,
    ) -> Result<Vec<MetaInstrument>, diesel::result::Error> {
        match instruments_table::table()
            .filter(instrument_quote_asset.eq(param_quote_asset))
            .load::<Instrument>(conn)
        {
            Ok(res) => {
                // Convert the Vec<Instrument> into Vec<MetaInstrument>
                Ok(res.iter().map(|i| i.to_meta_instrument()).collect())
            }
            Err(e) => Err(e),
        }
    }

    /// Queries all instruments for a specific base asset on a particular exchange.
    ///
    /// # Arguments
    /// - `conn`: A mutable reference to the database connection.
    /// - `param_base_asset`: The base asset to filter the instruments by.
    /// - `param_exchange_code`: The exchange code to filter the instruments by.
    ///
    /// # Returns
    /// A Result containing a vector of MetaInstrument if the query is successful, otherwise a diesel::result::Error.
    pub fn query_all_instruments_for_base_asset_on_exchange(
        conn: &mut Connection,
        param_base_asset: &str,
        param_exchange_code: &str,
    ) -> Result<Vec<MetaInstrument>, diesel::result::Error> {
        match instruments_table::table()
            .filter(instrument_base_asset.eq(param_base_asset))
            .filter(instrument_exchanges_code.eq(param_exchange_code))
            .load::<Instrument>(conn)
        {
            Ok(res) => {
                // Convert the Vec<Instrument> into Vec<MetaInstrument>
                Ok(res.iter().map(|i| i.to_meta_instrument()).collect())
            }
            Err(e) => Err(e),
        }
    }

    /// Queries all instruments for a specific quote asset on a particular exchange.
    ///
    /// # Arguments
    /// - `conn`: A mutable reference to the database connection.
    /// - `param_quote_asset`: The quote asset to filter the instruments by.
    /// - `param_exchange_code`: The exchange code to filter the instruments by.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of MetaInstrument if the query is successful,
    /// otherwise a diesel::result::Error.
    pub fn query_all_instruments_for_quote_asset_on_exchange(
        conn: &mut Connection,
        param_quote_asset: &str,
        param_exchange_code: &str,
    ) -> Result<Vec<MetaInstrument>, diesel::result::Error> {
        match instruments_table::table()
            .filter(instrument_quote_asset.eq(param_quote_asset))
            .filter(instrument_exchanges_code.eq(param_exchange_code))
            .load::<Instrument>(conn)
        {
            Ok(res) => {
                // Convert the Vec<Instrument> into Vec<MetaInstrument>
                Ok(res.iter().map(|i| i.to_meta_instrument()).collect())
            }
            Err(e) => Err(e),
        }
    }

    /// Queries all instruments for a specific base asset, quote asset, and exchange code.
    ///
    /// # Arguments
    /// - `conn`: A mutable reference to the database connection.
    /// - `param_base_asset`: The base asset to filter the instruments by.
    /// - `param_quote_asset`: The quote asset to filter the instruments by.
    /// - `param_exchange_code`: The exchange code to filter the instruments by.
    ///
    /// # Returns
    /// A Result containing a vector of MetaInstrument if successful, or a diesel::result::Error if an error occurs.
    pub fn query_all_instruments_for_base_quote_asset_on_exchange(
        conn: &mut Connection,
        param_base_asset: &str,
        param_quote_asset: &str,
        param_exchange_code: &str,
    ) -> Result<Vec<MetaInstrument>, diesel::result::Error> {
        match instruments_table::table()
            .filter(instrument_base_asset.eq(param_base_asset))
            .filter(instrument_quote_asset.eq(param_quote_asset))
            .filter(instrument_exchanges_code.eq(param_exchange_code))
            .load::<Instrument>(conn)
        {
            Ok(res) => {
                // Convert the Vec<Instrument> into Vec<MetaInstrument>
                Ok(res.iter().map(|i| i.to_meta_instrument()).collect())
            }
            Err(e) => Err(e),
        }
    }
}
