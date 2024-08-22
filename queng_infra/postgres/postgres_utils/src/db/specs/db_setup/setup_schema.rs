use crate::common::all_db_constants::SYSTEM_SCHEMA;
use crate::common::common_ddl::ddl_schema;
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Creates the schema for all specifications.
    ///
    /// This method is responsible for creating the schema for all specifications in the database.
    /// It performs the following steps:
    ///
    /// 1. Generates the DDL (Data Definition Language) for creating the schema using the `ddl_schema::generate_create_schema_ddl` function.
    /// 2. Executes the generated DDL using the `execute_query` method.
    ///
    /// If the schema creation is successful, it returns `Ok(())`.
    /// Otherwise, it returns an `Err` variant of `PostgresUtilError` with a descriptive error message.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the schema for all specifications is successfully created.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if the schema creation operation fails.
    ///
    pub(crate) async fn create_all_spec_schema(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_system_schema");
        let create_ddl = &ddl_schema::generate_create_schema_ddl(SYSTEM_SCHEMA);
        match self.execute_query(create_ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(format!(
                "Failed to create system schema: {}",
                e
            ))),
        }
    }
}
