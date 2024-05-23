use crate::errors::InitError;
use crate::fields::VALID_EXCHANGES_DOWNLOAD_FILE;
use crate::init::InitManager;
use crate::utils::{util_json, util_scraping};
use crate::VRB;

impl InitManager {
    pub(super) async fn init_level_0(&self) -> Result<Vec<String>, InitError> {
        //
        self.dbg_print("Scraping valid exchanges");
        let valid_exchanges = util_scraping::scrap_valid_exchanges(VRB)
            .await
            .expect("Error scraping valid exchanges");

        self.dbg_print("Saving valid exchanges to JSON file");
        util_json::save_to_json(&valid_exchanges, VALID_EXCHANGES_DOWNLOAD_FILE)
            .await
            .expect("Error saving valid exchanges to JSON file");

        if self.dbg {
            println!(
                "Saved {} exchanges to {}",
                valid_exchanges.len(),
                VALID_EXCHANGES_DOWNLOAD_FILE
            );
        }

        self.dbg_print("Done");

        Ok(valid_exchanges)
    }
}
