use crate::db::{Specs, DB_NAME};
use crate::prelude::PostgresUtilError;
use std::error::Error;

impl Specs {
    pub async fn create_spec_db(&mut self) -> Result<(), Box<PostgresUtilError>> {
        let ddl = format!("CREATE DATABASE IF NOT EXISTS {DB_NAME}");
        self.execute_query(ddl)
            .await
            .expect("Failed to drop specs DB");

        Ok(())
    }

    pub async fn verify_specs_db_exists(&self) -> Result<bool, Box<dyn Error>> {
        Ok(false)
    }

    pub async fn drop_spec_db(&mut self) -> Result<(), Box<dyn Error>> {
        let ddl = format!("DROP DATABASE IF EXISTS {DB_NAME}");
        self.execute_query(ddl)
            .await
            .expect("Failed to drop specs DB");

        Ok(())
    }
}
