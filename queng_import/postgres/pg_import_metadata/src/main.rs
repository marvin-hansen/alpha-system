mod print_utils;

use environment_manager::EnvironmentManager;
use kaiko_import::prelude::{determine_workflow, execute_workflow, WorkflowOpAll};
use mimalloc::MiMalloc;
use pg_mddb_manager::PostgresMDDBManager;
use postgres_config_manager::PostgresConfigManager;
use std::error::Error;
use tokio::time::Instant;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DBG: bool = true;
const AUTO_DETECT_PROXY: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start_main = Instant::now();
    print_utils::print_start_header();

    print_utils::dbg_print("Setup autoconfiguration");
    let config_manager = EnvironmentManager::new();
    let env_type = config_manager.env_type();
    print_utils::print_env(&env_type);

    print_utils::dbg_print("Configure postgres database");
    let pg_cfg_manager = PostgresConfigManager::new(&env_type);
    let dsn = pg_cfg_manager.pg_connection_url();
    let dbm_mddb = PostgresMDDBManager::new(&dsn)
        .await
        .expect("Failed to create PostgresSMDBManager");

    print_utils::dbg_print("Loading metadata records from Database");
    let start = Instant::now();
    let meta_data_db = dbm_mddb
        .count_metadata_records()
        .await
        .expect("Failed to load metadata from DB");
    print_utils::print_duration("Loading metadata from DB took", &start.elapsed());

    print_utils::dbg_print("Download metadata stats");
    let meta_data_stats = kaiko_download::download_meta_data_stats(DBG, AUTO_DETECT_PROXY)
        .await
        .expect("Failed to download metadata stats");

    print_utils::dbg_print("Determine workflow");
    let workflow = determine_workflow(&meta_data_stats, &meta_data_db).await;

    if workflow.all_op() == WorkflowOpAll::NoOPAll {
        print_utils::print_already_imported_header();
        print_utils::print_duration("Main took", &start_main.elapsed());
        return Ok(());
    }

    print_utils::dbg_print("Download metadata");
    let start = Instant::now();
    let meta_data = kaiko_download::download_meta_data(DBG, AUTO_DETECT_PROXY)
        .await
        .expect("Failed to download metadata");
    print_utils::print_duration("Downloading metadata took", &start.elapsed());

    print_utils::dbg_print("Import metadata into Database");
    let start = Instant::now();
    execute_workflow(&dbm_mddb, &meta_data, &workflow).await;
    print_utils::print_duration("Executing workflow took", &start.elapsed());

    print_utils::print_duration("Main took", &start_main.elapsed());

    Ok(())
}
