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
        self.dbg_print("Level 1: Download reference exchange data!");
        let downloaded_exchanges = utils::download_exchanges()
            .await
            .expect("Failed to download exchange data");

        self.dbg_print("Level 1: Process downloaded exchanges");
        let processed_exchanges = self
            .process_exchanges(valid_exchanges, downloaded_exchanges)
            .await
            .expect("Failed to process reference exchange data");

        if self.dbg {
            let msg = format!(
                "Level 1: Returning {} valid exchanges",
                processed_exchanges.len()
            );
            self.dbg_print(&msg)
        }

        Ok(processed_exchanges)
    }

    async fn process_exchanges(
        &self,
        valid_exchanges: &Vec<String>,
        downloaded_exchanges: Vec<Exchange>,
    ) -> Result<Vec<Exchange>, InitError> {
        let mut processed_exchanges = Vec::with_capacity(valid_exchanges.len());

        for e in downloaded_exchanges {
            if valid_exchanges.contains(&e.name.to_uppercase()) {
                processed_exchanges.push(e);
            }
        }

        Ok(processed_exchanges)
    }
}
