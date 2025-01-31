/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use klickhouse::Client;

mod db_schema;
mod db_setup;
mod db_teardown;
mod tables;
mod utils;

pub const ASSETS_TABLE: &str = "assets";
pub const EXCHANGES_TABLE: &str = "exchanges";
pub const INSTRUMENTS_TABLE: &str = "instruments";
pub const STATS_TABLE: &str = "stats";

pub const DB_TABLES: [&str; 4] = [
    ASSETS_TABLE,
    EXCHANGES_TABLE,
    INSTRUMENTS_TABLE,
    STATS_TABLE,
];

pub const DB_NAME: &str = "metadata";

#[derive(Clone)]
pub struct Metadata {
    dbg: bool,
    client: Client,
}

impl Metadata {
    pub const fn new(client: Client, dbg: bool) -> Self {
        Self { dbg, client }
    }

    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ClickhouseUtil]:[DB]:[Metadata]: {s}");
        }
    }
}
