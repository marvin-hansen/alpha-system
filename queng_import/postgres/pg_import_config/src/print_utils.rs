use crate::DBG;

pub(crate) fn print_start_header() {
    println!();
    println!("==========================================");
    println!("||  Migrating PostgresSQL Database ||",);
    println!("==========================================");
    println!();
}

pub(crate) fn print_already_header() {
    println!();
    println!("==========================================");
    println!("|| Database Already Migrated ||");
    println!("==========================================");
    println!();
}

pub(crate) fn print_stop_header(nr_of_configs: usize, schema_ok: bool) {
    println!();
    println!("||  Migrating PostgresSQL Database  ||");
    println!("==========================================");
    println!("Migrated PostgresSQL DB schemas: {}", schema_ok);
    println!("Imported Configurations in CMDB schema: {}", nr_of_configs);
    println!("Migration and Data Import Complete");
    println!("==========================================");
    println!();
}

pub(crate) fn dbg_print(msg: &str) {
    if DBG {
        println!("[main]: {}", msg)
    }
}
