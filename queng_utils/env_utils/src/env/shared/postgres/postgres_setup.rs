use crate::prelude::EnvironmentError;
use crate::EnvUtil;

impl EnvUtil {
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
