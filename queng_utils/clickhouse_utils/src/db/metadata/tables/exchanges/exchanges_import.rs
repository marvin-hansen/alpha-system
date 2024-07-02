use crate::db::metadata::Metadata;
use crate::types::error::ClickHouseUtilError;
use common::prelude::Exchange;

impl Metadata {
    /// Imports a list of exchanges metadata into the metadata database.
    ///
    /// This method takes a vector of `Exchange` objects and imports their metadata into the metadata database.
    /// It generates an insert query for each exchange using the `generate_exchange_insert` method and executes it using the `execute_query` method.
    ///
    /// # Arguments
    ///
    /// * `&self` - A reference to the `Metadata` object.
    /// * `exchanges` - A reference to a vector of `Exchange` objects containing the metadata to be imported.
    ///
    /// # Returns
    ///
    /// * `Result<(), ClickHouseUtilError>` - A result indicating the success of the import operation. Returns `Ok(())` if the import is successful, or an `Err` containing the error if it fails.
    ///
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
