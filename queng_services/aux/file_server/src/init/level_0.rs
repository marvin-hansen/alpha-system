use crate::errors::InitError;
use crate::init::InitManager;
use crate::utils::util_scraping;

impl InitManager {
    pub(super) async fn init_level_0(&self) -> Result<Vec<String>, InitError> {
        //
        self.dbg_print("Scraping valid exchanges");
        let valid_exchanges = util_scraping::scrap_valid_exchanges()
            .await
            .expect("Error scraping valid exchanges");

        Ok(valid_exchanges)
    }
}
