use crate::db::all_db_constants::SYSTEM_SCHEMA;
use crate::db::common_ddl::{ddl_schema, ddl_verify};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub(crate) async fn create_all_spec_schema(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_system_schema");
        let create_ddl = &ddl_schema::generate_create_schema_ddl(SYSTEM_SCHEMA);
        return match self.execute_query(create_ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(format!(
                "Failed to create system schema: {}",
                e.to_string()
            ))),
        };
    }

    pub(crate) async fn verify_all_spec_schema_exists(&self) -> Result<bool, PostgresUtilError> {
        self.dbg_print("verify_system_schema_exists");
        let verify_ddl = &ddl_verify::generate_verify_schema_ddl(SYSTEM_SCHEMA);
        return match self.execute_verify_query(verify_ddl).await {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresUtilError::new(format!(
                "Failed to verify system schema: {}",
                e.to_string()
            ))),
        };
    }
}
