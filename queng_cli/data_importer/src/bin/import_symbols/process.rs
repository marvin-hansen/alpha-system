use client_utils::print_utils;
use klickhouse::Client;
use lib_import::types::assets::AssetRoot;
use lib_import::types::exchanges::{Exchange, ExchangesRoot};
use lib_import::types::instruments::{Instrument, InstrumentsRoot};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub(crate) async fn process(
    client: &Client,
    file_path: &PathBuf,
    vrb: bool,
) -> Result<(), Box<dyn Error>> {
    //
    let file = file_path
        .file_name()
        .expect("Failed to get file name")
        .to_str()
        .expect("Failed to convert file name to str")
        .replace(".json", "");

    match file.as_str() {
        "assets" => {
            process_assets(client, file_path, vrb)
                .await
                .expect("Failed to process assets");
        }

        "exchanges" => {
            process_exchanges(client, file_path, vrb)
                .await
                .expect("Failed to process exchanges");
        }

        "instruments" => {
            process_instruments(client, file_path, vrb)
                .await
                .expect("Failed to process instruments");
        }

        &_ => {
            println!("Unknown file: {}", file);
        }
    }

    Ok(())
}

async fn process_assets(
    client: &Client,
    file_path: &PathBuf,
    vrb: bool,
) -> Result<(), Box<dyn Error>> {
    print_utils::dbg_print(vrb, "Processing assets");

    let file = File::open(file_path).expect("file not found");

    let assets: AssetRoot = serde_json::from_reader(file).expect("error while reading");

    let count = assets.data.len();

    println!("Number of assets: {}", count);

    Ok(())
}

async fn process_exchanges(
    client: &Client,
    file_path: &PathBuf,
    vrb: bool,
) -> Result<(), Box<dyn Error>> {
    print_utils::dbg_print(vrb, "Processing exchanges");

    let file = File::open(file_path).expect("file not found");

    let exchanges: ExchangesRoot = serde_json::from_reader(file).expect("error while reading");

    let count = exchanges.data.len();

    println!("Number of exchanges: {}", count);

    Ok(())
}

async fn process_instruments(
    client: &Client,
    file_path: &PathBuf,
    vrb: bool,
) -> Result<(), Box<dyn Error>> {
    print_utils::dbg_print(vrb, "Processing instruments");
    let exchange_gone = Arc::new(AtomicUsize::new(0));
    let instrument_inactive = Arc::new(AtomicUsize::new(0));

    let file = File::open(file_path).expect("file not found");

    let instruments: InstrumentsRoot = serde_json::from_reader(file).expect("error while reading");

    let mut filtered = Vec::new();

    for instrument in instruments.data.iter() {
        if is_valid_instrument(instrument, instrument_inactive.clone()) {
            filtered.push(instrument.clone());
        }
    }

    let count = instruments.data.len();
    println!("Number of All instruments: {}", count);

    let number_of_instruments_inactive = instrument_inactive.load(Ordering::SeqCst);
    println!(
        "Number of instruments inactive: {}",
        number_of_instruments_inactive
    );

    let count = filtered.len();
    println!(
        "Number of Filtered (active, non-option, etc) instruments: {}",
        count
    );

    Ok(())
}

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
    if instrument.class.contains("option") {
        return false;
    }

    true
}

fn is_active_exchange(exchange: &Exchange) -> bool {
    let inactive_exchanges = vec![""];

    if inactive_exchanges.contains(&exchange.name.as_str()) {
        return false;
    }

    true
}
