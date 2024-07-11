use crate::error::KaikoUtilError;
use crate::KaikoUtil;
use common::prelude::{MetaAsset, MetaExchange, MetaInstrument, Stats};

impl KaikoUtil {
    pub async fn get_assets(&self) -> Result<Vec<MetaAsset>, KaikoUtilError> {
        self.dbg_print("[get_assets]: Download asset metadata from Kaiko.");
        match self.client.get_assets().await {
            Ok(assets) => Ok(assets),
            Err(e) => Err(KaikoUtilError::new(&format!(
                "Error retrieving assets: {}",
                e
            ))),
        }
    }

    pub async fn get_exchanges(&self) -> Result<Vec<MetaExchange>, KaikoUtilError> {
        self.dbg_print("[get_exchanges]: Download exchange metadata from Kaiko.");
        return match self.client.get_exchanges().await {
            Ok(exchanges) => Ok(exchanges),
            Err(e) => {
                return Err(KaikoUtilError::new(&format!(
                    "Error retrieving exchanges: {}",
                    e
                )))
            }
        };
    }

    pub async fn get_stats(&self) -> Result<Stats, KaikoUtilError> {
        self.dbg_print("[get_instruments]: Download metadata statistics from Kaiko.");

        match self.client.get_stats().await {
            Ok(stats) => Ok(stats),

            Err(e) => Err(KaikoUtilError::new(&format!(
                "Error retrieving metadata statistics: {}",
                e
            ))),
        }
    }

    pub async fn get_instruments(&self) -> Result<Vec<MetaInstrument>, KaikoUtilError> {
        self.dbg_print("[get_instruments]: Download instrument metadata from Kaiko.");

        return match self.client.get_instruments().await {
            Ok(instruments) => Ok(instruments),
            Err(e) => Err(KaikoUtilError::new(&format!(
                "Error retrieving instruments: {}",
                e
            ))),
        };
    }
}
