use crate::db::specs::Specs;
use crate::query_utils;
use std::error::Error;

impl Specs {
    pub(crate) async fn create_spec_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = self.create_specs_db_ddl();
        query_utils::execute_query(&self.client, &ddl)
            .await
            .expect("Failed to drop specs DB");

        Ok(())
    }
    fn create_specs_db_ddl(&self) -> String {
        "CREATE DATABASE IF NOT EXISTS specs".to_string()
    }
}

impl Specs {
    pub async fn drop_spec_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = self.drop_specs_ddl();
        query_utils::execute_query(&self.client, &ddl)
            .await
            .expect("Failed to drop specs DB");

        Ok(())
    }

    fn drop_specs_ddl(&self) -> String {
        "DROP DATABASE IF EXISTS specs".to_string()
    }
}
