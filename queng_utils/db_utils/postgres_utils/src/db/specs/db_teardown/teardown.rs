use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Tears down the specifications database.
    ///
    /// This method tears down the specifications database by performing the following steps:
    /// 1. Drops all relation tables using the `drop_all_relation_tables` method.
    /// 2. Drops all specs tables using the `drop_all_specs_tables` method.
    /// 3. Drops the specs schema using the `drop_all_specs_schema` method.
    /// 4. Drops the specs database itself using the `drop_spec_db` method.
    ///
    /// If any of the teardown steps fail, an error is returned describing the cause of the failure.
    ///
    /// # Arguments
    ///
    /// * `drop`: A boolean flag indicating whether to drop the databases at the end of the teardown.
    ///   If `true`, the databases will be dropped. If `false`, the databases will be preserved.
    ///
    /// Note, even if the database is preserved, all tables, types, and schemas will be dropped.
    ///
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the specifications database is successfully torn down.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if any of the teardown operations fail.
    ///
    pub async fn teardown_spec_db(&self, drop: bool) -> Result<(), PostgresUtilError> {
        self.dbg_print("teardown_spec_db");

        self.dbg_print("[teardown_spec_db]: drop_all_relation_tables");
        match self.drop_all_relation_tables().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to drop all relation tables: {}",
                    e.to_string()
                )))
            }
        }

        self.dbg_print("[teardown_spec_db]: drop_all_specs_tables");
        match self.drop_all_specs_tables().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to drop all specs tables: {}",
                    e.to_string()
                )))
            }
        }

        self.dbg_print("[teardown_spec_db]: drop_all_specs_schema");
        match self.drop_all_specs_schema().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to drop all specs schema: {}",
                    e.to_string()
                )))
            }
        }

        if drop {
            self.dbg_print("[teardown_spec_db]: drop_spec_db");
            match self.drop_spec_db().await {
                Ok(_) => (),
                Err(e) => {
                    return Err(PostgresUtilError::new(format!(
                        "Error: Failed to drop specs DB: {}",
                        e.to_string()
                    )))
                }
            }
        }

        Ok(())
    }
}
