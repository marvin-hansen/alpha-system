use crate::error::ClickHouseUtilError;
use crate::types::{Exchange, ExchangesRoot};
use crate::ClickhouseUtil;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

impl ClickhouseUtil {
    pub(crate) async fn import_exchanges_data(
        &self,
        path: &str,
    ) -> Result<(), ClickHouseUtilError> {
        let exchanges = self
            .load_exchanges(path)
            .await
            .expect("Failed to load exchange.json");

        for exchange in exchanges.iter() {
            if exchange.active {
                let insert_query = self.metadata.generate_exchange_insert(exchange);
                self.execute_query(&insert_query)
                    .await
                    .expect("Failed to insert asset")
            }
        }

        Ok(())
    }

    async fn load_exchanges(&self, path: &str) -> Result<Vec<Exchange>, ClickHouseUtilError> {
        let file_path = PathBuf::from(path);
        let file = File::open(file_path).expect("file not found");
        let exchanges: ExchangesRoot = serde_json::from_reader(file).expect("error while reading");
        Ok(exchanges.data)
    }
}
