use crate::errors::InitError;
use crate::init::InitManager;
use crate::utils;
use common::prelude::{Asset, Exchange};

impl InitManager {
    pub(super) async fn init_level_2_assets(
        &self,
        valid_exchanges: &Vec<Exchange>,
    ) -> Result<Vec<Asset>, InitError> {
        //
        let downloaded_assets = self
            .download_assets()
            .await
            .expect("Failed to download reference asset data");

        //
        let valid_assets = self
            .process_assets(valid_exchanges, &downloaded_assets)
            .await
            .expect("Failed to process reference asset data");

        Ok(valid_assets)
    }

    async fn download_assets(&self) -> Result<Vec<Asset>, InitError> {
        utils::download_assets()
            .await
            .expect("Failed to download asset data");

        Ok(Vec::new())
    }

    async fn process_assets(
        &self,
        _valid_exchanges: &Vec<Exchange>,
        _downloaded_assets: &Vec<Asset>,
    ) -> Result<Vec<Asset>, InitError> {
        Ok(Vec::new())
    }
}
