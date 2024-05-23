mod init;
mod level_0;
mod level_1;
mod level_2;

use serde::{Deserialize, Serialize};

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
}
