use kaiko_import::WorkflowOpAll;
use metadata_import::MetadataImportManager;
use mimalloc::MiMalloc;
use std::error::Error;
use tokio::time::Instant;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DBG: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start_main = Instant::now();
    pg_import_print_utils::print_start_header();

    pg_import_print_utils::dbg_print(DBG, "Construct MetadataImportManager");
    let meta_data_import_manager = if DBG {
        MetadataImportManager::with_debug().await
    } else {
        MetadataImportManager::new().await
    };

    pg_import_print_utils::dbg_print(DBG, "Determine workflow");
    let workflow = meta_data_import_manager
        .determine_workflow(None)
        .await
        .expect("Failed to determine workflow");

    if workflow.all_op() == WorkflowOpAll::NoOPAll {
        pg_import_print_utils::print_already_imported_header();
        pg_import_print_utils::print_duration(DBG, "Main took", &start_main.elapsed());
        return Ok(());
    }

    pg_import_print_utils::dbg_print(DBG, "Import metadata into Database");
    let start = Instant::now();
    meta_data_import_manager
        .execute_workflow(&workflow)
        .await
        .expect("Failed to execute workflow");
    pg_import_print_utils::print_duration(DBG, "Executing workflow took", &start.elapsed());

    pg_import_print_utils::print_duration(DBG, "Main took", &start_main.elapsed());

    Ok(())
}
