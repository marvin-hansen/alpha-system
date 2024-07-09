use crate::ClickhouseUtil;
use std::error::Error;

impl ClickhouseUtil {
    pub async fn setup_all_tables(&self) -> Result<(), Box<dyn Error>> {
        self.metadata
            .create_all_metadata_tables()
            .await
            .expect("[setup_db]: Failed to create metadata DB");

        Ok(())
    }

    pub async fn drop_all_tables(&self) -> Result<(), Box<dyn Error>> {
        self.metadata
            .drop_metadata_tables()
            .await
            .expect("[teardown_db]: Failed to drop metadata DB");
        Ok(())
    }
}
