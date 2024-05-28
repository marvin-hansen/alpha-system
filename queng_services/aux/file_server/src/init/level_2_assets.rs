use crate::errors::InitError;
use crate::init::InitManager;
use crate::utils;
use common::prelude::Asset;

impl InitManager {
    pub(super) async fn init_level_2_assets(&self) -> Result<Vec<Asset>, InitError> {
        // Download the assets data
        let downloaded_assets = utils::download_assets()
            .await
            .expect("Failed to download asset data");

        if self.dbg {
            let msg = format!("Returning {} valid assets", downloaded_assets.len());
            self.dbg_print(&msg)
        }

        Ok(downloaded_assets)
    }
}
