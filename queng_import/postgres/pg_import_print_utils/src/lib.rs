/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use std::time::Duration;

pub fn print_start_header() {
    println!();
    println!("==========================================");
    println!("||  Importing PostgresSQL Database ||",);
    println!("==========================================");
    println!();
}

pub fn print_already_imported_header() {
    println!();
    println!("==========================================");
    println!("|| Data Already Imported ||");
    println!("==========================================");
    println!();
}

pub fn print_stop_header(nr_of_item: usize, name_of_items: &str, import_ok: bool) {
    println!();
    println!("||  Importing PostgresSQL Database  ||");
    println!("==========================================");
    println!("Migrated PostgresSQL DB: {import_ok}");
    println!("Imported {nr_of_item} {name_of_items} into database");
    println!("Data Import Complete");
    println!("==========================================");
    println!();
}

pub fn print_duration(dbg: bool, msg: &str, elapsed: &Duration) {
    if dbg {
        if elapsed.as_millis() > 1000 {
            println!("[pg_import_metadata]: {} {} sec.", msg, elapsed.as_secs());
        } else {
            println!("[pg_import_metadata]: {} {} ms.", msg, elapsed.as_millis());
        }
    }
}

pub fn dbg_print(dbg: bool, msg: &str) {
    if dbg {
        println!("[main]: {msg}");
    }
}
