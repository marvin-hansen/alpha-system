use crate::types::Exchange;

pub fn generate_exchange_insert(exchange: &Exchange) -> String {
    let table_name = "default.exchanges";
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
