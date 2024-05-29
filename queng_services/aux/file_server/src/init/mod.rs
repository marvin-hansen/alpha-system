mod init;
mod level_0;
mod level_1_exchanges;
mod level_2_assets;
mod level_3_instruments;
mod level_4_symbol_mapping;

use serde::{Deserialize, Serialize};
use service_utils::print_utils;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitManager {
    dbg: bool,
}

impl InitManager {
    pub fn new(dbg: bool) -> Self {
        Self { dbg }
    }
}

impl InitManager {
    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[InitManager]: {}", s);
        }
    }

    fn print_duration(&self, msg: &str, elapsed: &Duration) {
        if self.dbg {
            let msg = format!("[InitManager]: {}", msg);
            print_utils::print_duration(msg.as_str(), elapsed);
            println!("[InitManager]:");
        }
    }
}
