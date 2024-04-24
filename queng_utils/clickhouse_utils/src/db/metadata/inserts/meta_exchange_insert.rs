use crate::db::metadata::{Metadata, TABLE_NAME};
use crate::types::Exchange;

impl Metadata {
    pub fn generate_exchange_insert(&self, exchange: &Exchange) -> String {
        let table_name = format!("{TABLE_NAME}.exchanges");
        let code = &exchange.code;
        let name = &exchange.name;
        let active = exchange.active;
        let url = &exchange.url.clone().unwrap_or("".to_string());
        format!(
            r"
        INSERT INTO {table_name} (*)
        VALUES (
        '{code}',
        '{name}',
         {active},
        '{url}'
         )
         "
        )
    }
}
