use crate::db::all_db_constants::DB_NAME;
use crate::db::ddl::ddl_db;
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn create_spec_db(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_spec_db");
        match self.drop_db(DB_NAME).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to drop specs DB: {}",
                    e.to_string()
                )))
            }
        };

        let create_ddl = &ddl_db::generate_create_db_ddl(DB_NAME);
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
        let verify_ddl = &ddl_db::generate_verify_db_ddl(DB_NAME);
        match self.execute_verify_query(verify_ddl, DB_NAME).await {
            Ok(res) => Ok(res),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to verify if specs DB exists: {}",
                    e.to_string()
                )))
            }
        }
    }
}
