use crate::error::KaikoUtilError;
use kaiko_client::KaikoClient;

mod error;
mod getters;
pub mod prelude;

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
        let kaiko_client = KaikoClient::with_url(API_PROXY_URL, true);
        let client = kaiko_client
            .unwrap_or_else(|_| panic!("Failed to construct KaikoClient with local PROXY"));
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
