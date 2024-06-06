use crate::errors::InitError;
use crate::init::InitManager;
use crate::types::meta_data_set::MetaDataSet;
use tokio::time::Instant;

impl InitManager {
    pub async fn init(&self, update: bool) -> Result<MetaDataSet, InitError> {
        let s = if update { "UPDATE" } else { "INIT" };

        self.dbg_print("==========");
        self.dbg_print(&format!("START {}", s));
        self.dbg_print("==========");

        let start = Instant::now();
        self.dbg_print("Level 0: Retrieving list of valid exchanges!");
        let valid_exchanges = self
            .init_level_0()
            .await
            .expect("Failed to complete init level 0");

        self.print_duration("Level 0: took", &start.elapsed());

        let start = Instant::now();
        self.dbg_print("Level 1: Retrieving reference exchange data!");
        let exchanges_meta_data = self
            .init_level_1_exchanges(&valid_exchanges)
            .await
            .expect("Failed init level 1: Reference exchange data");

        self.print_duration("Level 1: took", &start.elapsed());

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
            .init_level_3_instruments()
            .await
            .expect("Failed init level 3: Reference instrument data");

        self.print_duration("Level 3: took", &start.elapsed());

        self.print_duration("Level 4: took", &start.elapsed());

        let meta_data =
            MetaDataSet::new(asset_meta_data, exchanges_meta_data, instrument_meta_data);

        self.dbg_print("=============");
        self.dbg_print(&format!("{} COMPLETE", s));
        self.dbg_print("=============");

        Ok(meta_data)
    }
}
