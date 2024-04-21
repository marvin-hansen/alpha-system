use client_utils::print_utils;
use db_utils::{
    ddl, insert, query_utils,
    types::{Exchange, ExchangesRoot},
};
use klickhouse::Client;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub(crate) async fn process_exchanges(
    client: &Client,
    file_path: &PathBuf,
    vrb: bool,
) -> Result<(), Box<dyn Error>> {
    print_utils::dbg_print(vrb, "Processing exchanges");
    let active = Arc::new(AtomicUsize::new(0));

    print_utils::dbg_print(vrb, "Load exchanges from file");
    let exchanges = get_exchanges_from_file(file_path)
        .await
        .expect("Failed to read exchanges from file");

    let ddl = ddl::generate_exchange_table_ddl();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to create exchanges table");

    print_utils::dbg_print(vrb, "Importing exchanges");
    for exchange in exchanges.iter() {
        if exchange.active {
            active.fetch_add(1, Ordering::SeqCst);
            let insert_query = insert::generate_exchange_insert(exchange);
            query_utils::execute_query(client, &insert_query)
                .await
                .expect("Failed to insert asset")
        }
    }

    let count = exchanges.len();
    println!("Number of exchanges: {}", count);
    let count = active.load(Ordering::SeqCst);
    println!("Number of active exchanges: {}", count);

    let count = query_utils::count_rows(client, "default.exchanges")
        .await
        .expect("Failed to count rows");
    println!("Number of exchanges imported: {}", count);

    Ok(())
}

async fn get_exchanges_from_file(file_path: &PathBuf) -> Result<Vec<Exchange>, Box<dyn Error>> {
    let file = File::open(file_path).expect("file not found");
    let exchanges: ExchangesRoot = serde_json::from_reader(file).expect("error while reading");
    Ok(exchanges.data)
}
