mod init_process;
mod level_1_exchanges;
mod level_2_assets;
mod level_3_instruments;

use crate::utils::util_download::DownloadUtils;
use service_utils::print_utils;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct InitManager {
    dbg: bool,
    dl_utils: DownloadUtils,
}

impl InitManager {
    pub fn new(dbg: bool) -> Self {
        Self {
            dbg,
            dl_utils: DownloadUtils::new(),
        }
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
