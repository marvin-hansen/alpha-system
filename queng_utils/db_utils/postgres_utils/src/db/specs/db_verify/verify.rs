use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Verifies if the specs database exists and all its schemas, tables and relation tables exist.
    ///
    /// This method performs the following steps:
    ///
    /// 1. Verifies if the specs database exists.
    /// 2. Verifies if all schemas exist.
    /// 3. Verifies if all tables exist.
    /// 4. Verifies if all relation tables exist.
    ///
    /// # Returns
    ///
    /// Returns a `Result` that indicates whether the specs database exists and all its schemas, tables and relation tables exist.
    /// If successful, it returns `Ok(())`.
    /// If an error occurs, it returns `Err(PostgresUtilError)`.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `PostgresUtilError`.
    ///
    pub async fn verify_spec_db(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("verify_spec_db");

        self.dbg_print("[verify_spec_db]: Verify if specs DB exists");
        let db_exists = self
            .verify_spec_db_exists()
            .await
            .expect("Failed to verify if specs DB exists");

        if !db_exists {
            return Err(PostgresUtilError::new(
                "Specs DB does not exist".to_string(),
            ));
        }

        self.dbg_print("[verify_spec_db]: Verify if all specs schemas exist");
        let schemas_exist = self
            .verify_all_spec_schema_exists()
            .await
            .expect("Failed to verify if all specs schemas exist");

        if !schemas_exist {
            return Err(PostgresUtilError::new(
                "Specs schemas do not exist".to_string(),
            ));
        }

        self.dbg_print("[verify_spec_db]: Verify if all specs tables exist");
        let tables_exist = self
            .verify_all_spec_tables_exists()
            .await
            .expect("Failed to verify if all specs tables exist");

        if !tables_exist {
            return Err(PostgresUtilError::new(
                "Specs tables do not exist".to_string(),
            ));
        }

        self.dbg_print("[verify_spec_db]: Verify if all specs relation tables exist");
        let relation_tables_exist = self
            .verify_all_spec_relation_tables_exists()
            .await
            .expect("Failed to verify if all specs relation tables exist");

        if !relation_tables_exist {
            return Err(PostgresUtilError::new(
                "Specs relation tables do not exist".to_string(),
            ));
        }

        Ok(())
    }
}
