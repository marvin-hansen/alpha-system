use mimalloc::MiMalloc;
use service_import::ServiceImportManager;
use std::error::Error;
use std::process::exit;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pg_import_print_utils::print_start_header();

    let service_import_manager = ServiceImportManager::new().await;
    let already_imported = service_import_manager.check_if_already_imported().await;

    // If all services have already been imported, exit the program
    if already_imported {
        pg_import_print_utils::print_already_imported_header();
        exit(0);
    }

    // If nothing has been imported yet, import all services
    pg_import_print_utils::dbg_print(DBG, "Import services");
    service_import_manager
        .import_services()
        .await
        .expect("Failed to import services");

    let nr_of_services = service_import_manager.count_db_services().await;
    pg_import_print_utils::print_stop_header(nr_of_services, "ServiceConfig", true);

    Ok(())
}
