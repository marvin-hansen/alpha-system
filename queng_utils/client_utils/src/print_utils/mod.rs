use std::time::Duration;

pub fn print_example_header(example: &str) {
    println!();
    println!("==========================================");
    println!("Running example: {}", example);
    println!("==========================================");
    println!();
}

pub fn print_data_import_header(msg: &str) {
    println!();
    println!("Import Data: {}", msg);
    println!();
}

pub fn dbg_print(vrb: bool, msg: &str) {
    if vrb {
        println!();
        println!("{msg}");
        println!();
    }
}

pub fn print_duration(elapsed: &Duration) {
    println!("Program took {:?} seconds.", elapsed.as_secs());
    println!();
}
