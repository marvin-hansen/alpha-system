use crate::ClickhouseUtil;
use std::error::Error;

impl ClickhouseUtil {
    pub async fn setup_db(&self) -> Result<(), Box<dyn Error>> {
        self.create_spec_db()
            .await
            .expect("[setup_db]: Failed to create specs DB");

        self.create_metadata_db()
            .await
            .expect("[setup_db]: Failed to create metadata DB");

        Ok(())
    }

    async fn create_spec_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = self.specs.create_specs_db();
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop specs DB");

        Ok(())
    }

    async fn create_metadata_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = self.metadata.create_metadata_db();
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop metadata DB");

        Ok(())
    }
}
