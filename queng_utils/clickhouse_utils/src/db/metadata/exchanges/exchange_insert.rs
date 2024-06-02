use crate::db::metadata::{Metadata, TABLE_NAME};
use common::prelude::Exchange;

impl Metadata {
    pub fn generate_exchange_insert(&self, exchange: &Exchange) -> String {
        let table_name = format!("{TABLE_NAME}.exchanges");
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
