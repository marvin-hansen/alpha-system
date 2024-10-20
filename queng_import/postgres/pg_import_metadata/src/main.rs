mod print_utils;

use environment_manager::EnvironmentManager;
use mimalloc::MiMalloc;
use pg_mddb_manager::PostgresMDDBManager;
use postgres_config_manager::PostgresConfigManager;
use std::error::Error;
use std::process::exit;
use std::time::Duration;
use tokio::time::Instant;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DBG: bool = true;
const AUTO_DETECT_PROXY: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print_utils::print_start_header();

    dbg_print("Setup autoconfiguration");
    let config_manager = EnvironmentManager::new();
    let env_type = config_manager.env_type();
    println!("[main]: Environment type: {:?}", env_type);

    dbg_print("Configure postgres database");
    let pg_cfg_manager = PostgresConfigManager::new(&env_type);
    let dsn = pg_cfg_manager.pg_connection_url();

    let dbm_mddb = PostgresMDDBManager::new(&dsn)
        .await
        .expect("Failed to create PostgresSMDBManager");

    dbg_print("Download metadata");
    let start = Instant::now();
    let meta_data = kaiko_download::download_meta_data(DBG, AUTO_DETECT_PROXY)
        .await
        .expect("Failed to download metadata");

    print_duration("Downloading metadata took", &start.elapsed());

    let stats = meta_data.stats();
    let expected_asset_count = stats.number_assets() as usize;
    let expected_exchange_count = stats.number_exchanges() as usize;
    let expected_instrument_count = stats.number_instruments() as usize;

    dbg_print("Check if assets already imported");
    let db_asset_count = dbm_mddb
        .count_assets()
        .await
        .expect("Failed to count assets") as usize;

    if db_asset_count == expected_asset_count {
        dbg_print("Assets already imported");
    }

    dbg_print("Check if exchanges already imported");
    let db_exchange_count = dbm_mddb
        .count_exchanges()
        .await
        .expect("Failed to count exchanges") as usize;

    if db_exchange_count == expected_exchange_count {
        dbg_print("Exchanges already imported");
    }

    dbg_print("Check if instruments already imported");
    let db_instrument_count = dbm_mddb
        .count_instruments()
        .await
        .expect("Failed to count instruments") as usize;

    if db_instrument_count == expected_instrument_count {
        dbg_print("Instruments already imported");
    }

    if db_asset_count == expected_asset_count
        && db_exchange_count == expected_exchange_count
        && db_instrument_count == expected_instrument_count
    {
        print_utils::print_already_imported_header();
        exit(0);
    }

    dbg_print("Importing metadata");
    let start = Instant::now();

    if db_asset_count == 0 {
        dbg_print("Import assets");
        let assets = meta_data.assets().data.as_slice();
        dbm_mddb
            .insert_asset_collection(assets)
            .await
            .expect("Failed to import assets");
    }

    if db_exchange_count == 0 {
        dbg_print("Import exchanges");
        let exchanges = meta_data.exchanges().data.as_slice();
        dbm_mddb
            .insert_exchange_collection(exchanges)
            .await
            .expect("Failed to import exchanges");
    }

    if db_instrument_count == 0 {
        dbg_print("Import instruments");
        let instruments = meta_data.instruments().data.as_slice();
        dbm_mddb
            .insert_instrument_collection(instruments)
            .await
            .expect("Failed to import instruments");

        // Inserts m to n relation that links exchanges with instruments
        // dbm_mddb
        //     .insert_instruments_exchanges_collection(instruments)
        //     .await
        //     .expect("Failed to import instruments_exchanges");
    }

    print_duration("Importing metadata took", &start.elapsed());

    let asset_count = dbm_mddb
        .count_assets()
        .await
        .expect("Failed to count assets") as usize;

    let exchange_count = dbm_mddb
        .count_exchanges()
        .await
        .expect("Failed to count exchanges") as usize;

    let instrument_count = dbm_mddb
        .count_instruments()
        .await
        .expect("Failed to count instruments") as usize;

    print_utils::print_stop_header(asset_count, exchange_count, instrument_count);

    Ok(())
}

fn dbg_print(msg: &str) {
    if DBG {
        println!("[main]: {}", msg)
    }
}

fn print_duration(msg: &str, elapsed: &Duration) {
    if DBG {
        let msg = format!("[main]: {}", msg);
        print_utils::print_duration(msg.as_str(), elapsed);
        println!("[main]:");
    }
}
