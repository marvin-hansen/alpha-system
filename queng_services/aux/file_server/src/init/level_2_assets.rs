use crate::errors::InitError;
use crate::init::InitManager;
use crate::utils;
use common::prelude::Asset;

impl InitManager {
    pub(super) async fn init_level_2_assets(&self) -> Result<Vec<Asset>, InitError> {
        //
        self.dbg_print("Level 2: Download reference asset data!");
        let downloaded_assets = get_assets().await;

        if self.dbg {
            let msg = format!(
                "Level 2: Returning {} valid assets",
                downloaded_assets.len()
            );
            self.dbg_print(&msg)
        }

        Ok(downloaded_assets)
    }
}

async fn get_assets() -> Vec<Asset> {
    let downloaded_assets = utils::download_assets()
        .await
        .expect("Failed to download asset data");

    downloaded_assets
}
