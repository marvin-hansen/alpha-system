use crate::error::ClickHouseUtilError;
use crate::ClickhouseUtil;
use common::prelude::Exchange;

impl ClickhouseUtil {
    pub async fn import_exchanges_metadata(
        &self,
        exchanges: &Vec<Exchange>,
    ) -> Result<(), ClickHouseUtilError> {
        for exchange in exchanges.iter() {
            let insert_query = self.metadata.generate_exchange_insert(exchange);
            self.execute_query(&insert_query)
                .await
                .expect("Failed to insert asset")
        }

        Ok(())
    }
}
