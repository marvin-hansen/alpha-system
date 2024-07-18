use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Sets up the specifications database.
    ///
    /// This method sets up the specifications database by performing the following steps:
    /// 1. Creates the specifications database using the `create_spec_db` method.
    /// 2. Verifies if the specifications database exists using the `verify_spec_db_exists` method.
    /// 3. Creates the schema for all specifications using the `create_all_spec_schema` method.
    /// 4. Verifies if the schema for all specifications exists using the `verify_all_spec_schema_exists` method.
    /// 5. Creates the tables for all specifications using the `create_all_specs_tables` method.
    /// 6. Verifies if the tables for all specifications exist using the `verify_all_spec_tables_exists` method.
    /// 7. Creates the relation tables for all specifications using the `create_all_specs_relation_tables` method.
    /// 8. Verifies if the relation tables for all specifications exist using the `verify_all_spec_relation_tables_exists` method.
    ///
    /// If any of the above steps fail, an error is returned describing the cause of the failure.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the specifications database is successfully set up.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if any of the setup operations fail.
    ///
    pub async fn setup_spec_db(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("setup_spec_db");

        match self.create_spec_db().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to create specs DB: {}",
                    e.to_string()
                )))
            }
        }

        match self.verify_spec_db_exists().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to verify if specs DB exists: {}",
                    e.to_string()
                )))
            }
        };

        match self.create_all_spec_schema().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to create all specs DB schema: {}",
                    e.to_string()
                )))
            }
        }

        match self.verify_all_spec_schema_exists().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to verify if all specs DB schema exists: {}",
                    e.to_string()
                )))
            }
        }

        match self.create_all_specs_tables().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to create all specs tables: {}",
                    e.to_string()
                )))
            }
        };

        match self.verify_all_spec_tables_exists().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to verify if all specs tables exists: {}",
                    e.to_string()
                )))
            }
        }

        match self.create_all_specs_relation_tables().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to create all specs relation tables: {}",
                    e.to_string()
                )))
            }
        }

        match self.verify_all_spec_relation_tables_exists().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to verify if all specs relation tables exists: {}",
                    e.to_string()
                )))
            }
        }

        Ok(())
    }
}
