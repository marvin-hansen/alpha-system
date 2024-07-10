use crate::db::specs::Specs;
use crate::db::specs::DB_NAME;
use std::error::Error;

impl Specs {
    pub async fn create_spec_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = format!("CREATE DATABASE IF NOT EXISTS {DB_NAME}");
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop specs DB");

        Ok(())
    }

    pub async fn verify_specs_db_exists(&self) -> Result<bool, Box<dyn Error>> {
        let exists = self
            .verify_db_exists(DB_NAME)
            .await
            .expect("Failed to verify if metadata DB");

        Ok(exists)
    }
}
