use kaiko_client::KaikoClient;

mod error;
mod getters;
mod inactive_exchanges;

pub use crate::error::KaikoUtilError;

pub struct KaikoUtil {
    dbg: bool,
    client: KaikoClient,
}

impl KaikoUtil {
    pub fn new() -> Result<Self, KaikoUtilError> {
        Self::build(false)
    }

    pub fn with_debug() -> Result<Self, KaikoUtilError> {
        Self::build(true)
    }

    fn build(dbg: bool) -> Result<Self, KaikoUtilError> {
        let kaiko_client = KaikoClient::new();
        let client = kaiko_client.expect("Failed to construct KaikoClient");
        Ok(Self { dbg, client })
    }
}

impl Default for KaikoUtil {
    fn default() -> Self {
        Self::new().expect("Failed to create KeikoUtil")
    }
}

impl KaikoUtil {
    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[DockerUtil]: {}", s);
        }
    }
}
