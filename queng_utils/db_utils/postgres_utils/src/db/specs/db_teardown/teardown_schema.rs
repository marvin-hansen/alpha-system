use crate::db::all_db_constants::SYSTEM_SCHEMA;
use crate::db::common_ddl::ddl_schema;
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn drop_all_specs_schema(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_all_specs_schema");

        self.dbg_print("/drop_all_specs_schema: drop_system_schema");
        let drop_ddl = &ddl_schema::generate_drop_schema_ddl(SYSTEM_SCHEMA);
        match self.execute_query(drop_ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(format!(
                "Failed to drop system schema: {}",
                e
            ))),
        }
    }
}
