use crate::ClickhouseUtil;
use std::error::Error;

impl ClickhouseUtil {
    pub async fn setup_db(&self) -> Result<(), Box<dyn Error>> {
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
}
