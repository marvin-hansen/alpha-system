use crate::init::InitManager;
use common_errors::prelude::InitError;
use common_metadata::prelude::MetaDataSet;
use tokio::time::Instant;

impl InitManager {
    /// Asynchronously initializes the InitManager by retrieving reference exchange,
    /// asset, and instrument data in three levels.
    /// Returns a Result containing MetaDataSet on success, or InitError on failure.
    pub async fn init(&self) -> Result<MetaDataSet, InitError> {
        let start = Instant::now();
        self.dbg_print("Level 1: Retrieving reference exchange data!");
        let exchanges_meta_data = self
            .init_level_1_exchanges()
            .await
            .expect("Failed init level 1: Reference exchange data");

        self.print_duration("Level 1: took", &start.elapsed());

        // valid_exchanges stores all exchanges codes as lowercase in a &str array
        let valid_exchanges = exchanges_meta_data
            .iter()
            .map(|x| x.code.to_lowercase())
            .collect::<Vec<String>>();

        let start = Instant::now();
        self.dbg_print("Level 2: Retrieving reference asset data!");
        let asset_meta_data = self
            .init_level_2_assets()
            .await
            .expect("Failed init level 2: Reference asset data");

        self.print_duration("Level 2: took", &start.elapsed());

        let start = Instant::now();
        self.dbg_print("Level 3: Retrieving reference instrument data!");
        let instrument_meta_data = self
            .init_level_3_instruments(&valid_exchanges)
            .await
            .expect("Failed init level 3: Reference instrument data");

        self.print_duration("Level 3: took", &start.elapsed());

        let meta_data =
            MetaDataSet::new(asset_meta_data, exchanges_meta_data, instrument_meta_data);

        self.dbg_print("=============");
        self.dbg_print("INIT COMPLETE");
        self.dbg_print("=============");

        Ok(meta_data)
    }
}
