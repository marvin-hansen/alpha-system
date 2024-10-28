use crate::DBG;
use common_env::prelude::EnvironmentType;
use std::time::Duration;

pub(crate) fn print_start_header() {
    println!();
    println!("==========================================");
    println!("||  Importing Metadata into PostgreSQL ||",);
    println!("==========================================");
    println!();
}

pub(crate) fn print_already_imported_header() {
    println!();
    println!("==========================================");
    println!("|| Metadata Already Migrated ||");
    println!("==========================================");
    println!();
}

pub(crate) fn print_stop_header(
    asset_count: usize,
    exchange_count: usize,
    instrument_count: usize,
) {
    println!();
    println!("||  Importing Metadata into PostgreSQL ||",);
    println!("==========================================");
    println!("Imported assets in MDDB schema: {}", asset_count);
    println!("Imported exchanges in MDDB schema: {}", exchange_count);
    println!("Imported instruments in MDDB schema: {}", instrument_count);
    println!("Migration and Data Import Complete");
    println!("==========================================");
    println!();
}

pub(crate) fn print_import_header(msg: &str, count: usize) {
    println!();
    println!("||  Importing Metadata into PostgreSQL ||",);
    println!("==========================================");
    println!("{}: {}", msg, count);
    println!("==========================================");
    println!();
}

pub(crate) fn print_duration(msg: &str, elapsed: &Duration) {
    if DBG {
        if elapsed.as_millis() > 1000 {
            println!("[pg_import_metadata]: {} {} sec.", msg, elapsed.as_secs());
        } else {
            println!("[pg_import_metadata]: {} {} ms.", msg, elapsed.as_millis());
        }
    }
}

pub(crate) fn print_env(env_type: &EnvironmentType) {
    println!("[pg_import_metadata]: Detected environment: {}", env_type)
}

pub(crate) fn dbg_print(msg: &str) {
    if DBG {
        println!("[pg_import_metadata]: {}", msg)
    }
}
