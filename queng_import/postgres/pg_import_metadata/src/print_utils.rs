use std::time::Duration;

pub fn print_start_header() {
    println!();
    println!("==========================================");
    println!("||  Importing Metadata into PostgreSQL ||",);
    println!("==========================================");
    println!();
}

pub fn print_already_imported_header() {
    println!();
    println!("==========================================");
    println!("|| Metadata Already Migrated ||");
    println!("==========================================");
    println!();
}

pub fn print_stop_header(asset_count: usize, exchange_count: usize, instrument_count: usize) {
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

pub fn print_duration(msg: &str, elapsed: &Duration) {
    if elapsed.as_millis() > 1000 {
        println!("{} {} sec.", msg, elapsed.as_secs());
    } else {
        println!("{} {} ms.", msg, elapsed.as_millis());
    }
}
