mod print_utils;

use kaiko_import::prelude::WorkflowOpAll;
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
    print_utils::print_start_header();

    print_utils::dbg_print("Construct MetadataImportManager");
    let meta_data_import_manager = if DBG {
        MetadataImportManager::with_debug().await
    } else {
        MetadataImportManager::new().await
    };

    print_utils::dbg_print("Determine workflow");
    let workflow = meta_data_import_manager
        .determine_workflow(None)
        .await
        .expect("Failed to determine workflow");

    if workflow.all_op() == WorkflowOpAll::NoOPAll {
        print_utils::print_already_imported_header();
        print_utils::print_duration("Main took", &start_main.elapsed());
        return Ok(());
    }

    print_utils::dbg_print("Import metadata into Database");
    let start = Instant::now();
    meta_data_import_manager
        .execute_workflow(&workflow)
        .await
        .expect("TODO: panic message");
    print_utils::print_duration("Executing workflow took", &start.elapsed());

    print_utils::print_duration("Main took", &start_main.elapsed());

    Ok(())
}
