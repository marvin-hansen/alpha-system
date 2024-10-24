use crate::fields::ACTIVE_EXCHANGES;
use crate::init::InitManager;
use common_errors::prelude::InitError;
use common_metadata::prelude::MetaExchange;

impl InitManager {
    pub(super) async fn init_level_1_exchanges(&self) -> Result<Vec<MetaExchange>, InitError> {
        //
        self.dbg_print("Level 1: Download reference exchange data!");
        let downloaded_exchanges = self
            .dl_utils
            .download_exchanges()
            .await
            .expect("Failed to download exchange data");

        if self.use_proxy {
            if self.dbg {
                let msg = format!(
                    "Level 1: Returning {} downloaded exchanges",
                    downloaded_exchanges.len()
                );
                self.dbg_print(&msg)
            }
            return Ok(downloaded_exchanges);
        }

        self.dbg_print("Level 1: Process downloaded exchanges");
        let processed_exchanges = process_exchanges(&downloaded_exchanges)
            .await
            .expect("Failed to process reference exchange data");

        if self.dbg {
            let msg = format!(
                "Level 1: Returning {} processed exchanges",
                processed_exchanges.len()
            );
            self.dbg_print(&msg)
        }

        drop(downloaded_exchanges);

        Ok(processed_exchanges)
    }
}

async fn process_exchanges(
    downloaded_exchanges: &[MetaExchange],
) -> Result<Vec<MetaExchange>, InitError> {
    let mut processed_exchanges: Vec<MetaExchange> = Vec::with_capacity(50);

    for e in downloaded_exchanges.iter() {
        if ACTIVE_EXCHANGES.contains(&e.name.to_lowercase().as_str()) {
            processed_exchanges.push(e.to_owned());
        }
    }

    // drop(downloaded_exchanges);

    // Remove duplicates
    processed_exchanges.dedup();

    // Sort alphabetically
    processed_exchanges.sort();

    Ok(processed_exchanges)
}
