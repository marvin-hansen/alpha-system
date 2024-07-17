use crate::db::{Specs, DB_NAME};
use crate::prelude::PostgresUtilError;
use crate::query_utils::ddl_utils;
use std::error::Error;

impl Specs {
    pub async fn create_spec_db(&mut self) -> Result<(), Box<PostgresUtilError>> {
        let drop_ddl = &ddl_utils::generate_drop_db_ddl(DB_NAME);
        self.execute_query(drop_ddl)
            .await
            .expect("Failed to drop specs DB");

        let create_ddl = &ddl_utils::generate_create_db_ddl(DB_NAME);
        self.execute_query(create_ddl)
            .await
            .expect("Failed to drop specs DB");

        Ok(())
    }

    pub async fn verify_spec_db_exists(&mut self) -> Result<bool, Box<dyn Error>> {
        let verify_ddl = &ddl_utils::generate_verify_db_ddl(DB_NAME);
        self.execute_verify_query(verify_ddl, DB_NAME)
            .await
            .expect("Failed to drop specs DB");

        Ok(true)
    }

    pub async fn drop_spec_db(&mut self) -> Result<(), Box<dyn Error>> {
        let drop_ddl = &ddl_utils::generate_drop_db_ddl(DB_NAME);
        self.execute_query(drop_ddl)
            .await
            .expect("Failed to drop specs DB");

        Ok(())
    }
}
