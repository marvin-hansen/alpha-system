use crate::db::{Specs, DB_NAME};
use crate::prelude::PostgresUtilError;
use crate::query_utils::ddl_utils;

impl Specs {
    pub async fn create_spec_db(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_spec_db");

        match self.drop_spec_db().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to drop specs DB: {}",
                    e.to_string()
                )))
            }
        };

        let create_ddl = &ddl_utils::generate_create_db_ddl(DB_NAME);
        return match self.execute_query(create_ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(format!(
                "Failed to create specs DB: {}",
                e.to_string()
            ))),
        };
    }

    pub async fn verify_spec_db_exists(&self) -> Result<bool, PostgresUtilError> {
        self.dbg_print("verify_spec_db_exists");
        let verify_ddl = &ddl_utils::generate_verify_db_ddl(DB_NAME);
        match self.execute_verify_query(verify_ddl, DB_NAME).await {
            Ok(res) => Ok(res),
            Err(e) => return Err(PostgresUtilError::new(e.to_string())),
        }
    }

    pub async fn drop_spec_db(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_spec_db");
        let drop_ddl = &ddl_utils::generate_drop_db_ddl(DB_NAME);
        match self.execute_query(drop_ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to drop specs DB: {}",
                    e.to_string()
                )))
            }
        };

        Ok(())
    }
}
