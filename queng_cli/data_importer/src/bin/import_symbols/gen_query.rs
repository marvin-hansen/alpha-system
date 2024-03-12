use lib_import::types::exchanges::Exchange;

pub fn generate_exchange_insert(exchange: &Exchange) -> String {
    let table_name = "default.exchanges";
    let code = exchange.code.clone();
    let name = exchange.name.clone();
    let active = exchange.active;
    let url = exchange.url.clone().unwrap_or("".to_string());
    format!(
        r"
        INSERT INTO {table_name} (*)
        VALUES (
        '{code}',
        '{name}',
         {active},
         '{url}'
         );
    "
    )
}
