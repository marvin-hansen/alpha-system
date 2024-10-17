mod print_utils;

use environment_manager::EnvironmentManager;
use mimalloc::MiMalloc;
use pg_mddb_manager::PostgresMDDBManager;
use postgres_config_manager::PostgresConfigManager;
use std::error::Error;
use std::process::exit;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print_utils::print_start_header();

    dbg_print("Setup autoconfiguration");
    let config_manager = EnvironmentManager::new();
    let env_type = config_manager.env_type();
    println!("[main]: Environment type: {:?}", env_type);

    dbg_print("Configure postgres database manager");
    let pg_cfg_manager = PostgresConfigManager::new(&env_type);
    let dsn = pg_cfg_manager.pg_connection_url();

    let dbm_mddb = PostgresMDDBManager::new(&dsn)
        .await
        .expect("Failed to create PostgresSMDBManager");

    dbg_print("Configure postgres util");

    // Check if metadata already imported and if not, import them.

    // Check if the stats exists, then read it, and extract the expected number of values.
    // Then compare the actual number to the expected number.

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

    if db_asset_count > 0 || db_exchange_count > 0 || db_instrument_count > 0 {
        print_utils::print_already_header();
        exit(0);
    }

    if db_asset_count == 0 || db_exchange_count == 0 || db_instrument_count == 0 {
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
