use crate::errors::InitError;
use crate::fields::ACTIVE_EXCHANGES;
use crate::init::InitManager;
use common::prelude::Exchange;

impl InitManager {
    pub(super) async fn init_level_1_exchanges(&self) -> Result<Vec<Exchange>, InitError> {
        //
        self.dbg_print("Level 1: Download reference exchange data!");
        let downloaded_exchanges = self
            .dl_utils
            .download_exchanges()
            .await
            .expect("Failed to download exchange data");

        if self.dbg {
            let msg = format!(
                "Level 1: Returning {} downloaded exchanges",
                downloaded_exchanges.len()
            );
            self.dbg_print(&msg)
        }

        self.dbg_print("Level 1: Process downloaded exchanges");
        let processed_exchanges = process_exchanges(downloaded_exchanges)
            .await
            .expect("Failed to process reference exchange data");

        if self.dbg {
            let msg = format!(
                "Level 1: Returning {} processed exchanges",
                processed_exchanges.len()
            );
            self.dbg_print(&msg)
        }

        Ok(processed_exchanges)
    }
}

async fn process_exchanges(
    downloaded_exchanges: Vec<Exchange>,
) -> Result<Vec<Exchange>, InitError> {
    let mut processed_exchanges = Vec::with_capacity(50);

    for e in downloaded_exchanges {
        if ACTIVE_EXCHANGES.contains(&e.name.as_str()) {
            processed_exchanges.push(e);
        }
    }

    // Remove duplicates
    processed_exchanges.dedup();

    // Sort alphabetically
    processed_exchanges.sort();

    Ok(processed_exchanges)
}
