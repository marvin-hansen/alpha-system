use crate::errors::InitError;
use crate::init::InitManager;
use crate::utils;
use common::prelude::{Asset, Exchange};

impl InitManager {
    pub(super) async fn init_level_2_assets(
        &self,
        valid_exchanges: &Vec<Exchange>,
    ) -> Result<Vec<Asset>, InitError> {
        // Download the assets data
        self.download_assets()
            .await
            .expect("Failed to download reference asset data");

        // Load the assets data from the downloaded file
        let downloaded_assets = self
            .load_assets()
            .await
            .expect("Failed to download reference asset data");

        // Process the downloaded assets
        let valid_assets = self
            .process_assets(valid_exchanges, &downloaded_assets)
            .await
            .expect("Failed to process reference asset data");

        Ok(valid_assets)
    }

    async fn download_assets(&self) -> Result<(), InitError> {
        utils::download_assets()
            .await
            .expect("Failed to download asset data");

        Ok(())
    }

    async fn load_assets(&self) -> Result<Vec<Asset>, InitError> {
        let assets = utils::load_assets()
            .await
            .expect("Failed to load assets from download file");
        Ok(assets)
    }

    async fn process_assets(
        &self,
        _valid_exchanges: &Vec<Exchange>,
        _downloaded_assets: &Vec<Asset>,
    ) -> Result<Vec<Asset>, InitError> {
        Ok(Vec::new())
    }
}
