use integration_import::IntegrationImportManager;
use mimalloc::MiMalloc;
use std::error::Error;
use std::process::exit;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pg_import_print_utils::print_start_header();

    let config_import_manager = IntegrationImportManager::with_test_and_debug().await;

    let already_imported = config_import_manager.check_if_integrations_imported().await;

    // If all integrations have already been imported, exit the program
    if already_imported {
        pg_import_print_utils::print_already_imported_header();
        exit(0);
    }

    // If nothing has been imported yet, import all services
    pg_import_print_utils::dbg_print(DBG, "Import integrations");
    config_import_manager
        .import_integration_configs()
        .await
        .expect("Failed to import integrations");

    // Count the number of imported integrations
    let nr_of_integrations = config_import_manager.count_db_integrations().await;
    pg_import_print_utils::print_stop_header(nr_of_integrations, "IntegrationConfig", true);

    Ok(())
}
