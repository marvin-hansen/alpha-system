use db_utils::prelude::ClickHouseClient;
use db_utils::types::Exchange;
use db_utils::{insert, query_utils};
use std::error::Error;

pub(crate) async fn import_exchanges(
    client: &ClickHouseClient,
    exchanges: &Vec<Exchange>,
) -> Result<(), Box<dyn Error>> {
    for exchange in exchanges.iter() {
        if exchange.active {
            let insert_query = insert::generate_exchange_insert(exchange);
            query_utils::execute_query(client, &insert_query)
                .await
                .expect("Failed to insert asset")
        }
    }

    Ok(())
}
