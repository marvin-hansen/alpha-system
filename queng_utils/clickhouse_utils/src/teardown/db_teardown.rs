use crate::ClickhouseUtil;
use std::error::Error;

impl ClickhouseUtil {
    pub async fn teardown_db(&self) -> Result<(), Box<dyn Error>> {
        self.specs
            .drop_spec_db()
            .await
            .expect("[teardown_db]: Failed to drop specs DB");

        self.metadata
            .drop_metadata_db()
            .await
            .expect("[teardown_db]: Failed to drop metadata DB");

        Ok(())
    }
}
