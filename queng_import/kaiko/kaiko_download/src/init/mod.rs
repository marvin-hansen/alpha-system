use std::time::Duration;

use common_service::print_utils;

use crate::utils::util_download::DownloadUtils;

mod init_process;
mod level_1_exchanges;
mod level_2_assets;
mod level_3_instruments;

#[derive(Debug, Clone)]
pub struct InitManager {
    dbg: bool,
    dl_utils: DownloadUtils,
}

impl InitManager {
    pub(crate) fn new(dbg: bool) -> Self {
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
