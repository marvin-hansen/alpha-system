use crate::init::InitManager;
use common_errors::InitError;
use common_metadata::{MetaDataSet, MetaStats};
use tokio::time::Instant;

impl InitManager {
    /// Retrieves reference exchange data, which includes a hash of all assets, exchanges, and instruments.
    /// This data is used to determine if the data in the database has changed.
    /// Returns a Result containing `MetaStats` on success, or `InitError` on failure.
    pub async fn get_meta_data_stats(&self) -> Result<MetaStats, InitError> {
        match self.dl_utils.download_stats().await {
            Ok(meta_stats) => Ok(meta_stats),
            Err(e) => Err(InitError::new(e.to_string())),
        }
    }

    /// Asynchronously initializes the `InitManager` by retrieving reference exchange,
    /// asset, and instrument data in three levels.
    /// Returns a Result containing `MetaDataSet` on success, or `InitError` on failure.
    pub async fn init(&self) -> Result<MetaDataSet, InitError> {
        self.dbg_print("");
        self.dbg_print("=================");
        self.dbg_print(" START DOWNLOAD ");
        self.dbg_print("=================");
        self.dbg_print("");

        let start = Instant::now();
        self.dbg_print("Level 1: Retrieving reference exchange data!");
        let exchanges_meta_data = self
            .init_level_1_exchanges()
            .await
            .expect("Failed init level 1: Reference exchange data");

        self.print_duration("Level 1: took", &start.elapsed());

        // valid_exchanges stores all exchanges codes as lowercase in a String array
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

        self.dbg_print("=================");
        self.dbg_print("DOWNLOAD COMPLETE");
        self.dbg_print("=================");
        self.dbg_print("");

        Ok(meta_data)
    }
}
