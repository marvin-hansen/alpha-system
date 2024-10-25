use std::time::Duration;

use crate::fields::BASE_URL;
use crate::utils::DownloadUtils;
use common_service::print_utils;

mod init_process;
mod level_1_exchanges;
mod level_2_assets;
mod level_3_instruments;
mod patch_instruments;
mod patch_op;
mod patches;

#[derive(Debug, Clone)]
pub struct InitManager {
    dbg: bool,
    use_proxy: bool,
    dl_utils: DownloadUtils,
}

impl InitManager {
    pub(crate) fn new(dbg: bool) -> Self {
        Self {
            dbg,
            use_proxy: false,
            dl_utils: DownloadUtils::new(BASE_URL),
        }
    }

    pub(crate) fn with_proxy_url(dbg: bool, proxy_url: &str) -> Self {
        Self {
            dbg,
            use_proxy: true,
            dl_utils: DownloadUtils::new(proxy_url),
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
