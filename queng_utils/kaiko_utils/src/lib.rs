use crate::error::KaikoUtilError;
use common::prelude::{AssetRoot, ExchangesRoot, InstrumentsRoot};

mod error;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct KaikoUtil {
    dbg: bool,
}

impl KaikoUtil {
    pub fn new() -> Result<Self, KaikoUtilError> {
        Self::build(false)
    }

    pub fn with_debug() -> Result<Self, KaikoUtilError> {
        Self::build(true)
    }

    fn build(dbg: bool) -> Result<Self, KaikoUtilError> {
        Ok(Self { dbg })
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

impl KaikoUtil {
    pub fn get_assets(&self) -> Result<AssetRoot, KaikoUtilError> {
        self.dbg_print("[get_assets]: Download asset metadata from Kaiko.");

        return Ok(AssetRoot::default());
    }

    pub fn get_exchanges(&self) -> Result<ExchangesRoot, KaikoUtilError> {
        self.dbg_print("[get_exchanges]: Download exchange metadata from Kaiko.");

        return Ok(ExchangesRoot::default());
    }

    pub fn get_instruments(&self) -> Result<InstrumentsRoot, KaikoUtilError> {
        self.dbg_print("[get_instruments]: Download instrument metadata from Kaiko.");

        return Ok(InstrumentsRoot::default());
    }
}
