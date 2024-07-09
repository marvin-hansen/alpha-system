use crate::ClickhouseUtil;
use std::error::Error;

impl ClickhouseUtil {
    pub async fn setup_all_db(&self) -> Result<(), Box<dyn Error>> {
        self.metadata
            .create_metadata_db()
            .await
            .expect("[setup_db]: Failed to create metadata DB");

        self.specs
            .create_spec_db()
            .await
            .expect("[setup_db]: Failed to create specs DB");

        Ok(())
    }

    pub async fn teardown_all_db(&self) -> Result<(), Box<dyn Error>> {
        self.metadata
            .drop_metadata_db()
            .await
            .expect("[teardown_db]: Failed to drop metadata DB");

        self.specs
            .drop_spec_db()
            .await
            .expect("[teardown_db]: Failed to drop specs DB");

        Ok(())
    }
}
