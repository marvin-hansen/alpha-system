use crate::db::metadata::{Metadata, DB_NAME};
use common::prelude::Exchange;

impl Metadata {
    /// Generates a ClickHouse SQL query to insert an exchange into the exchanges table.
    ///
    /// This method generates a SQL query that can be used to insert an exchange into the exchanges table.
    ///
    /// # Arguments
    ///
    /// * `exchange` - A reference to an `Exchange` object that contains the exchange data to be inserted.
    ///
    /// # Returns
    ///
    /// * `String` - The generated SQL query.
    ///
    pub(crate) fn generate_exchange_insert(&self, exchange: &Exchange) -> String {
        let table_name = format!("{DB_NAME}.exchanges");
        let code = &exchange.code;
        let name = &exchange.name;
        format!(
            r"
        INSERT INTO {table_name} (*)
        VALUES (
        '{code}',
        '{name}',
         )
         "
        )
    }
}
