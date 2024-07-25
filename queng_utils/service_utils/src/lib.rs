use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use common_config::prelude::ServiceID;
use ctx_manager::CtxManager;

mod error;
pub mod prelude;
mod service_start;
mod service_stop;
mod utils;
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ServiceUtil {
    dbg: bool,
    ctx_manager: CtxManager,
    services: RefCell<HashMap<ServiceID, String>>,
}

impl ServiceUtil {
    pub fn new() -> Self {
        Self::build(false)
    }

    pub fn with_debug() -> Self {
        Self::build(true)
    }

    pub fn build(dbg: bool) -> Self {
        if dbg {
            println!("[ServiceUtil]: Debug mode enabled");
        }

        let ctx_manager = if dbg {
            CtxManager::with_debug()
        } else {
            CtxManager::new()
        };

        Self {
            dbg,
            ctx_manager,
            services: RefCell::new(HashMap::new()),
        }
    }
}

impl ServiceUtil {
    pub fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ServiceUtil]: {}", s);
        }
    }
}

impl Display for ServiceUtil {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceUtil {{ debug mode: {} }}", &self.dbg)
    }
}
