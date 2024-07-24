use crate::prelude::EnvironmentError;
use crate::EnvUtil;

impl EnvUtil {
    /// Configures the PostgreSQL database for the environment.
    ///
    /// This method configures the PostgreSQL database for the environment by performing the following steps:
    ///
    /// 1. Retrieves a new `PostgresUtil` object asynchronously.
    ///
    /// 2. Sets up all databases if they do not already exist using the `setup_all_db` method of the `PostgresUtil` object.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the PostgreSQL database is configured successfully,
    /// or an `Err` variant of `EnvironmentError` if an error occurs during the configuration process.
    ///
    /// # Errors
    ///
    /// Returns an `EnvironmentError` if any of the following errors occur during the configuration process:
    ///
    /// - If there is an error retrieving the Postgres utility.
    /// - If there is an error creating the databases.
    ///
    pub async fn setup_postgres(&self) -> Result<(), EnvironmentError> {
        self.dbg_print("setup_postgres");

        self.dbg_print("[setup_postgres]; Get Postgres util");
        let pg_util = self
            .get_new_postgres_util()
            .await
            .expect("[setup_postgres]: Failed to get PostgresUtil");

        self.dbg_print("[setup_postgres]: Setup all databases");
        pg_util
            .setup_all_db()
            .await
            .expect("[setup_postgres]: Failed to create databases");

        let data_imported = self
            .verify_postgres_data_imported(&pg_util)
            .await
            .expect("[setup_postgres]: Failed to check if all data imported");

        self.dbg_print(&format!(
            "[setup_postgres]: Data imported: {}",
            data_imported
        ));
        if data_imported {
            // If so, abort & return. Nothing to do in this case.
            self.dbg_print(&"[setup_postgres]:Nothing to configure or import; return.".to_string());

            return Ok(());
        }

        self.dbg_print("[setup_postgres]: Import data into Postgres");
        self.import_all_pg_data(&pg_util)
            .await
            .expect("[setup_postgres]: Failed to import data into Postgres");

        Ok(())
    }
}
