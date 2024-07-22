use crate::db::all_db_constants::SYSTEM_SCHEMA;
use crate::db::common_ddl::{ddl_schema, ddl_verify};
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

    /// Verifies if the schema for all specifications exists.
    ///
    /// This method is responsible for verifying if the schema for all specifications exists in the database.
    /// It performs the following steps:
    ///
    /// 1. Generates the DDL (Data Definition Language) for verifying the schema using the `ddl_verify::generate_verify_schema_ddl` function.
    /// 2. Executes the generated DDL using the `execute_verify_query` method.
    ///
    /// If the schema verification is successful, it returns `Ok(true)`.
    /// Otherwise, it returns an `Err` variant of `PostgresUtilError` with a descriptive error message.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the schema for all specifications exists.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if the schema verification operation fails.
    ///
    pub(crate) async fn verify_all_spec_schema_exists(&self) -> Result<bool, PostgresUtilError> {
        self.dbg_print("verify_system_schema_exists");
        let verify_ddl = &ddl_verify::generate_verify_schema_ddl(SYSTEM_SCHEMA);
        match self.execute_verify_query(verify_ddl).await {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresUtilError::new(format!(
                "Failed to verify system schema: {}",
                e
            ))),
        }
    }
}
