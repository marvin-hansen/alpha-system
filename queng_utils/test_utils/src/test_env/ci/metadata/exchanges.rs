use db_utils::prelude::ClickHouseClient;
use db_utils::types::{Exchange, ExchangesRoot};
use db_utils::{ddl, insert, query_utils};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub(crate) async fn setup_exchanges_table(client: &ClickHouseClient) -> Result<(), Box<dyn Error>> {
    let ddl = ddl::generate_create_exchanges_table_ddl();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to create exchanges table");

    Ok(())
}

pub(crate) async fn import_exchanges(client: &ClickHouseClient) -> Result<(), Box<dyn Error>> {
    let path = "";
    let exchanges = load_exchanges(path)
        .await
        .expect("Failed to load exchange.json");

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

async fn load_exchanges(path: &str) -> Result<Vec<Exchange>, Box<dyn Error>> {
    let file_path = PathBuf::from(path);
    let file = File::open(file_path).expect("file not found");
    let exchanges: ExchangesRoot = serde_json::from_reader(file).expect("error while reading");
    Ok(exchanges.data)
}
