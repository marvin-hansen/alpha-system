use crate::ClickhouseUtil;
use std::error::Error;

impl ClickhouseUtil {
    pub async fn setup_all_tables(&self) -> Result<(), Box<dyn Error>> {
        self.dbg_print("[setup_all_tables]: create all metadata tables");
        self.metadata
            .create_all_metadata_tables()
            .await
            .expect("[setup_db]: Failed create all metadata tables");

        self.specs
            .create_all_specs_tables()
            .await
            .expect("[setup_db]: Failed to create all specs tables");

        Ok(())
    }

    pub async fn drop_all_tables(&self) -> Result<(), Box<dyn Error>> {
        self.dbg_print("[drop_all_tables]: Create all metadata tables");
        self.metadata
            .drop_all_metadata_tables()
            .await
            .expect("[teardown_db]: Failed to drop all metadata tables");

        self.dbg_print("[drop_all_tables]: Create all specs tables");
        self.specs
            .drop_all_specs_tables()
            .await
            .expect("[teardown_db]: Failed to drop all specs tables");

        Ok(())
    }
}
