pub fn print_start_header() {
    println!();
    println!("==========================================");
    println!("||  Migrating PostgresSQL Database ||",);
    println!("==========================================");
    println!();
}

pub fn print_already_header() {
    println!();
    println!("==========================================");
    println!("|| Database Already Migrated ||",);
    println!("==========================================");
    println!();
}

pub fn print_stop_header(nr_of_services: usize, schema_ok: bool) {
    println!();
    println!("||  Migrating PostgresSQL Database  ||");
    println!("==========================================");
    println!("Migrated PostgresSQL DB schemas: {}", schema_ok);
    println!("Imported services in SMDB schema: {}", nr_of_services);
    println!("Migration and Data Import Complete");
    println!("==========================================");
    println!();
}
