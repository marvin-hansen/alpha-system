use crate::errors::InitError;
use crate::init::InitManager;
use common::prelude::MetaAsset;

impl InitManager {
    pub(super) async fn init_level_2_assets(&self) -> Result<Vec<MetaAsset>, InitError> {
        //
        self.dbg_print("Level 2: Download reference asset data!");
        let downloaded_assets = self
            .dl_utils
            .download_assets()
            .await
            .expect("Failed to download asset data");

        if self.dbg {
            let msg = format!("Level 2: Returning {} assets", downloaded_assets.len());
            self.dbg_print(&msg)
        }

        Ok(downloaded_assets)
    }
}
