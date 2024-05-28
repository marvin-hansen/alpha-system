use crate::errors::InitError;
use crate::init::InitManager;
use crate::types::meta_data_set::MetaDataSet;

impl InitManager {
    pub async fn init(&self) -> Result<MetaDataSet, InitError> {
        //
        self.dbg_print("Level 0: Retrieving list of valid exchanges!");
        let valid_exchanges = self
            .init_level_0()
            .await
            .expect("Failed to complete init level 0");

        self.dbg_print("Level 1: Retrieving reference exchange data!");
        let exchanges_meta_data = self
            .init_level_1_exchanges(&valid_exchanges)
            .await
            .expect("Failed init level 1: Reference exchange data");

        self.dbg_print("Level 2: Retrieving reference asset data!");
        let asset_meta_data = self
            .init_level_2_assets()
            .await
            .expect("Failed init level 2: Reference asset data");

        self.dbg_print("Level 3: Retrieving reference instrument data!");
        let instrument_meta_data = self
            .init_level_3_instruments()
            .await
            .expect("Failed init level 3: Reference instrument data");

        let meta_data = MetaDataSet::new(
            asset_meta_data,
            exchanges_meta_data,
            valid_exchanges,
            instrument_meta_data,
        );

        self.dbg_print("Init complete!");

        Ok(meta_data)
    }
}
