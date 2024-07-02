use crate::db::metadata::Metadata;
use crate::error::ClickHouseUtilError;
use common::prelude::Exchange;

impl Metadata {
    pub async fn import_exchanges_metadata(
        &self,
        exchanges: &Vec<Exchange>,
    ) -> Result<(), ClickHouseUtilError> {
        for exchange in exchanges.iter() {
            let insert_query = self.generate_exchange_insert(exchange);
            self.execute_query(&insert_query)
                .await
                .expect("Failed to insert asset")
        }

        Ok(())
    }
}
