use crate::DBG;
use pg_smdb_manager::PostgresSMDBManager;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) type SafePostgresDBManager = Arc<RwLock<PostgresSMDBManager>>;

#[derive(Clone)]
pub struct DBGWServer {
    dbg: bool,
    dbm: SafePostgresDBManager,
}

impl DBGWServer {
    pub fn new(dbm: SafePostgresDBManager) -> Self {
        Self { dbg: DBG, dbm }
    }
}

impl DBGWServer {
    pub fn dbm(&self) -> &SafePostgresDBManager {
        &self.dbm
    }
}
impl DBGWServer {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[DBGW/service]: {}", msg)
        }
    }
}
