use crate::common::all_db_constants::SYSTEM_SCHEMA;
use crate::common::common_ddl::ddl_schema;
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Drops all the schema related to specifications.
    ///
    /// This method is responsible for dropping all the schema related to specifications in the database.
    /// It performs the following steps:
    ///
    /// 1. Generates the DDL (Data Definition Language) for dropping the system schema using the `ddl_schema::generate_drop_schema_ddl` function.
    /// 2. Executes the generated DDL using the `execute_query` method.
    ///
    /// If the dropping operation is successful, it returns `Ok(())`. Otherwise, it returns an `Err` variant of `PostgresUtilError`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all the schema related to specifications are dropped successfully.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error while dropping the schema related to specifications.
    ///
    pub async fn drop_all_specs_schema(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_all_specs_schema");

        self.dbg_print("/drop_all_specs_schema: drop_system_schema");
        let drop_ddl = &ddl_schema::generate_drop_schema_ddl(SYSTEM_SCHEMA);
        match self.execute_query(drop_ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to drop system schema: {}",
                    e
                )))
            }
        };

        Ok(())
    }
}
