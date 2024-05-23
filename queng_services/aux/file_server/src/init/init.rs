use crate::errors::InitError;
use crate::init::InitManager;

impl InitManager {
    pub async fn init(&self) -> Result<(), InitError> {
        //
        self.dbg_print("Level 0: Scraping valid exchanges");
        let valid_exchanges = self
            .init_level_0()
            .await
            .expect("Failed to complete init level 0");

        self.dbg_print("Level 1: Downloading JSON files from API");
        self.init_level_1(valid_exchanges)
            .await
            .expect("Failed to complete init level 1");

        Ok(())
    }
}
