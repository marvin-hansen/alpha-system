use crate::errors::InitError;
use crate::init::InitManager;
use crate::utils;
use common::prelude::Exchange;

impl InitManager {
    pub(super) async fn init_level_1_exchanges(
        &self,
        valid_exchanges: &Vec<String>,
    ) -> Result<Vec<Exchange>, InitError> {
        //
        let downloaded_exchanges = self
            .download_exchanges()
            .await
            .expect("Failed to download exchange reference data");

        //
        let valid_exchanges = self
            .process_exchanges(valid_exchanges, &downloaded_exchanges)
            .await
            .expect("Failed to process reference exchange data");

        Ok(valid_exchanges)
    }

    async fn download_exchanges(&self) -> Result<Vec<Exchange>, InitError> {
        utils::download_exchanges()
            .await
            .expect("Failed to download exchange data");

        Ok(Vec::new())
    }

    async fn process_exchanges(
        &self,
        _valid_exchanges: &Vec<String>,
        _downloaded_exchanges: &Vec<Exchange>,
    ) -> Result<Vec<Exchange>, InitError> {
        Ok(Vec::new())
    }
}
