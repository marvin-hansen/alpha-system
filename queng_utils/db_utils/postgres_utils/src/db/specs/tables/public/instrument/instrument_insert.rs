use crate::db::Specs;
use crate::prelude::PostgresUtilError;
use common_exchange::prelude::Instrument;

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

impl Specs {
    pub(crate) async fn insert_instrument(
        &self,
        data: &Instrument,
    ) -> Result<u64, PostgresUtilError> {
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

    fn build_insert_instrument_query(&self, data: &Instrument) -> String {
        format!(
            "INSERT INTO system.instrument(code, class, exchange_code, exchange_pair_code, base_asset, quote_asset, instrument_figi)
             VALUES('{}', '{}', '{}', '{}', '{}', '{}', '{}')
             RETURNING id",
            data.code(), data.class(), data.exchange_code(), data.exchange_pair_code(), data.base_asset(), data.quote_asset(), data.instrument_figi().clone().unwrap_or_else(|| "null".to_string())
        )
    }
}
