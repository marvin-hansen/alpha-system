use crate::prelude::{EnvUtil, EnvironmentError};

impl EnvUtil {
    /// Sets up the Postgres database for testing.
    ///
    /// This function performs the following steps:
    ///
    /// 1. Checks if Postgres is already configured. If it is, the function returns early.
    /// 2. Retrieves the Postgres utilities.
    /// 3. Sets up all Postgres databases.
    /// 4. Verifies if all data has been imported into Postgres.
    /// 5. If data has not been imported, imports all data into Postgres.
    ///
    /// # Errors
    ///
    /// - `EnvironmentError` if any step fails.
    ///
    pub async fn setup_postgres(&self) -> Result<(), EnvironmentError> {
        self.dbg_print("setup_postgres");

        self.dbg_print("[setup_postgres]; Check if Postgres is already configured");
        if self.postgres_configured {
            self.dbg_print("[setup_postgres]; Postgres is already configured; return.");
            return Ok(());
        }

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
            self.dbg_print("[setup_postgres]:Nothing to configure or import; return.");

            return Ok(());
        }

        self.dbg_print("[setup_postgres]: Import data into Postgres");
        self.import_all_pg_data(&pg_util)
            .await
            .expect("[setup_postgres]: Failed to import data into Postgres");

        Ok(())
    }
}
