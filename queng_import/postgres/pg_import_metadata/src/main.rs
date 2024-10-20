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
const USE_PROXY: bool = true;

const PROXY_URL: Option<&str> = if USE_PROXY {
    Some("http://127.0.0.1:7777/")
} else {
    None
};

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
    let meta_data = kaiko_download::download_meta_data(DBG, PROXY_URL)
        .await
        .expect("Failed to download metadata");

    print_duration("Downloading metadata took", &start.elapsed());

    let stats = meta_data.stats();
    let expected_asset_count = stats.number_assets() as usize;
    let expected_exchange_count = stats.number_exchanges() as usize;
    let expected_instrument_count = stats.number_instruments() as usize;

    dbg_print("Check if metadata already imported");
    let db_asset_count = dbm_mddb
        .count_assets()
        .await
        .expect("Failed to count assets") as usize;

    let db_exchange_count = dbm_mddb
        .count_exchanges()
        .await
        .expect("Failed to count exchanges") as usize;

    let db_instrument_count = dbm_mddb
        .count_instruments()
        .await
        .expect("Failed to count instruments") as usize;

    if db_asset_count == expected_asset_count
        && db_exchange_count == expected_exchange_count
        && db_instrument_count == expected_instrument_count
    {
        print_utils::print_already_imported_header();
        exit(0);
    }

    if db_asset_count == 0 && db_exchange_count == 0 && db_instrument_count == 0 {
        let asset_count = 0;
        let exchange_count = 0;
        let instrument_count = 0;
        print_utils::print_stop_header(asset_count, exchange_count, instrument_count);
    }

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
