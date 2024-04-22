use db_utils::fields::INACTIVE_EXCHANGES;
use db_utils::prelude::ClickHouseClient;
use db_utils::types::{Instrument, InstrumentsRoot};
use db_utils::{ddl, insert, query_utils};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub(crate) async fn setup_instruments_table(
    client: &ClickHouseClient,
) -> Result<(), Box<dyn Error>> {
    let ddl = ddl::generate_create_instruments_table_ddl();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to create instruments table");

    Ok(())
}

async fn load_instruments(path: &str) -> Result<Vec<Instrument>, Box<dyn Error>> {
    let file_path = PathBuf::from(path);
    let file = File::open(file_path).expect("instruments.json file not found");
    let instruments: InstrumentsRoot = serde_json::from_reader(file).expect("error while reading");
    Ok(instruments.data)
}

pub(crate) async fn import_instruments(client: &ClickHouseClient) -> Result<(), Box<dyn Error>> {
    let path = "";
    let instruments = load_instruments(path)
        .await
        .expect("Failed to load instrument.json file");

    for instrument in instruments.iter() {
        // Skip all instruments from inactive exchanges
        if INACTIVE_EXCHANGES.contains(&instrument.exchange_code()) {
            continue;
        }

        if is_valid_instrument(instrument) {
            let insert_query = insert::generate_instruments_insert(instrument);
            query_utils::execute_query(client, &insert_query)
                .await
                .expect("Failed to insert asset")
        }
    }

    Ok(())
}

// Double check if instrument is inactive i.e. from an inactive exchange
fn is_valid_instrument(instrument: &Instrument) -> bool {
    // Instrument  inactive
    if instrument.trade_start_time.is_none() && instrument.trade_end_time.is_none() {
        return false;
    }

    // Instrument inactive
    if instrument.trade_end_time.is_some() && instrument.trade_end_timestamp.is_some() {
        return false;
    }

    // Instrument is of no interest
    if instrument.class.eq("option") {
        return false;
    }

    // Non-perpetual future contracts.
    if instrument.class.eq("future") {
        return false;
    }

    true
}
