use kaiko_client::KaikoClient;

mod error;
mod getters;
mod inactive_exchanges;

pub use crate::error::KaikoUtilError;

// Co-located API proxy speeds up data download.
const API_PROXY_URL: &str = "http://localhost:7777/";

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
        let kaiko_client = KaikoClient::with_url(API_PROXY_URL);
        let client = kaiko_client.expect(&format!(
            "Failed to construct KaikoClient for PROXY URL {}",
            API_PROXY_URL
        ));
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
            println!("[KaikoUtil]:{}", s);
        }
    }
}
