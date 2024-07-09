use crate::db::specs::Specs;
use crate::db::specs::DB_NAME;
use std::error::Error;

impl Specs {
    pub(crate) async fn create_spec_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = self.create_specs_db_ddl();
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop specs DB");

        Ok(())
    }
    fn create_specs_db_ddl(&self) -> String {
        "CREATE DATABASE IF NOT EXISTS specs".to_string()
    }

    pub async fn verify_specs_db_exists(&self) -> Result<bool, Box<dyn Error>> {
        let exists = self
            .verify_db_exists(DB_NAME)
            .await
            .expect("Failed to verify if metadata DB");

        Ok(exists)
    }
}
