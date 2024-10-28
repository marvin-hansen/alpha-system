use crate::DBG;

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

pub(crate) fn dbg_print(msg: &str) {
    if DBG {
        println!("[kaiko_import]: {}", msg)
    }
}
