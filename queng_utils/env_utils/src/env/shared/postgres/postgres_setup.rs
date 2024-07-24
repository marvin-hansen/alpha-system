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

        Ok(())
    }
}
