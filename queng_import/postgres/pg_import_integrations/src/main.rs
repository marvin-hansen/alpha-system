use mimalloc::MiMalloc;
use std::error::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pg_import_print_utils::print_start_header();

    Ok(())
}
