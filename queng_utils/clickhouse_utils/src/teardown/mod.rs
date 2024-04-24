use crate::ClickhouseUtil;
use std::error::Error;

impl ClickhouseUtil {
    pub async fn teardown(&self) -> Result<(), Box<dyn Error>> {
        self.drop_spec_db().await.expect("Failed to drop specs DB");

        self.drop_metadata_db()
            .await
            .expect("Failed to drop metadata DB");

        Ok(())
    }

    async fn drop_spec_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = self.specs.drop_specs_db();
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop specs DB");

        Ok(())
    }

    async fn drop_metadata_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = self.metadata.drop_metadata_db();
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop metadata DB");

        Ok(())
    }
}
