use common_exchange::prelude::Instrument;

use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Inserts a new instrument into the `public.instrument` table.
    ///
    /// # Arguments
    ///
    /// * `data` - The instrument to be inserted.
    ///
    /// # Returns
    ///
    /// Returns the ID of the inserted instrument on success.
    ///
    /// # Errors
    ///
    /// Returns a `PostgresUtilError` if the insertion fails.
    ///
    pub(crate) async fn insert_instrument(
        &self,
        data: &Instrument,
    ) -> Result<u64, PostgresUtilError> {
        self.dbg_print("insert_instrument");

        let query = self.build_insert_instrument_query(data);
        // println!("query: {}", query);
        match self.execute_insert_query(&query).await {
            Ok(id) => Ok(id),
            Err(err) => Err(PostgresUtilError::new(format!(
                "Failed to insert instrument: {} due error: {}",
                &data.code(),
                err
            ))),
        }
    }

    // insert into public.instrument(id,code ,"class" ,exchange_code,exchange_pair_code,base_asset,quote_asset,instrument_figi )
    // VALUES(
    // 1,
    // 'ens-krw',
    // 'spot',
    // 'cbse',
    // 'KRW-ENS',
    // 'ens',
    // 'krw',
    // null
    // )
    // RETURNING id;
    fn build_insert_instrument_query(&self, data: &Instrument) -> String {
        format!(
            "INSERT INTO public.instrument(code, class, exchange_code, exchange_pair_code, base_asset, quote_asset, instrument_figi)
             VALUES('{}', '{}', '{}', '{}', '{}', '{}', '{}')
             RETURNING id",
            data.code(), data.class(), data.exchange_code(), data.exchange_pair_code(), data.base_asset(), data.quote_asset(), data.instrument_figi().clone().unwrap_or_else(|| "null".to_string())
        )
    }
}
