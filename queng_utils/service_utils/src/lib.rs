use common::prelude::{ServiceConfig, ServiceID};
use service_specs::prelude::{
    cmdb_service_config, dbgw_service_config, ims_data_binance_config, qdgw_service_config,
    smdb_service_config, symdb_service_config, vex_service_config,
};
use std::fmt::{Debug, Display};

pub mod print_utils;
pub mod shutdown_utils;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ServiceUtil {
    dbg: bool,
}

impl ServiceUtil {
    pub fn new() -> Self {
        Self::build(false)
    }

    pub fn with_debug() -> Self {
        Self::build(true)
    }

    fn build(dbg: bool) -> Self {
        Self { dbg }
    }
}

impl Default for ServiceUtil {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceUtil {
    fn dbg_print(&self, s: &str, t: impl Display) {
        if self.dbg {
            println!("[ServiceUtil]: {} {}", s, t);
        }
    }
}
