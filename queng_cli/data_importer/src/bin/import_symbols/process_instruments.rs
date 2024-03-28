use crate::fields::INACTIVE_EXCHANGES;
use crate::gen_ddl;
use crate::gen_query::generate_instruments_insert;
use client_utils::print_utils;
use db_utils::query_utils;
use klickhouse::Client;
use lib_import::types::instruments::{Instrument, InstrumentsRoot};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub async fn process_instruments(
    client: &Client,
    file_path: &PathBuf,
    vrb: bool,
) -> Result<(), Box<dyn Error>> {
    print_utils::dbg_print(vrb, "Processing instruments");

    let instrument_inactive = Arc::new(AtomicUsize::new(0));
    let instrument_filtered = Arc::new(AtomicUsize::new(0));
    let instrument_figi_counter = Arc::new(AtomicUsize::new(0));
    let pair_figi_counter = Arc::new(AtomicUsize::new(0));

    print_utils::dbg_print(vrb, "Load instruments from file");
    let instruments = get_instruments_from_file(file_path)
        .await
        .expect("Failed to read instruments from file");

    let ddl = gen_ddl::generate_instruments_table_ddl();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to create instrument table");

    let ddl = gen_ddl::generate_master_symbols_table_ddl();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to create symbol master table");

    print_utils::dbg_print(vrb, "Insert instruments");
    for instrument in instruments.iter() {
        // Skip all instruments from inactive exchanges
        if INACTIVE_EXCHANGES.contains(&instrument.exchange_code()) {
            continue;
        }

        if is_valid_instrument(instrument, instrument_inactive.clone()) {
            if instrument.metadata.is_some() {
                let meta = instrument.metadata.as_ref().unwrap();
                if meta.instrument_figi.is_some() {
                    instrument_figi_counter.fetch_add(1, Ordering::SeqCst);
                }
                if meta.pair_figi.is_some() {
                    pair_figi_counter.fetch_add(1, Ordering::SeqCst);
                }
            }

            instrument_filtered.fetch_add(1, Ordering::SeqCst);
            let insert_query = generate_instruments_insert(instrument);
            query_utils::execute_query(client, &insert_query)
                .await
                .expect("Failed to insert asset")
        }
    }

    let count = instruments.len();
    println!("Number of All instruments: {}", count);

    let number_of_instruments_inactive = instrument_inactive.load(Ordering::SeqCst);
    println!(
        "Number of instruments inactive: {}",
        number_of_instruments_inactive
    );

    let count = instrument_filtered.load(Ordering::SeqCst);
    println!(
        "Number of filtered (active, non-option, etc) instruments: {}",
        count
    );

    let count = instrument_figi_counter.load(Ordering::SeqCst);
    println!(
        "Number of filtered instruments with instrument FIGI: {}",
        count
    );

    let count = pair_figi_counter.load(Ordering::SeqCst);
    println!("Number of filtered instruments with Pair FIGI: {}", count);

    let count = query_utils::count_rows(client, "default.instruments")
        .await
        .expect("Failed to count rows");
    println!("Number of instruments imported: {}", count);

    Ok(())
}

async fn get_instruments_from_file(file_path: &PathBuf) -> Result<Vec<Instrument>, Box<dyn Error>> {
    let file = File::open(file_path).expect("file not found");
    let instruments: InstrumentsRoot = serde_json::from_reader(file).expect("error while reading");
    Ok(instruments.data)
}

// Double check if instrument is inactive i.e. from an inactive exchange
fn is_valid_instrument(instrument: &Instrument, instrument_inactive: Arc<AtomicUsize>) -> bool {
    // Instrument  inactive
    if instrument.trade_start_time.is_none() && instrument.trade_end_time.is_none() {
        instrument_inactive.fetch_add(1, Ordering::SeqCst);
        return false;
    }

    // Instrument inactive
    if instrument.trade_end_time.is_some() && instrument.trade_end_timestamp.is_some() {
        instrument_inactive.fetch_add(1, Ordering::SeqCst);
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
