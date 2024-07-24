use crate::prelude::EnvironmentSetupError;
use crate::EnvUtil;

impl EnvUtil {
    pub async fn setup_all_containers(&self) -> Result<(), EnvironmentSetupError> {
        //
        self.dbg_print("[setup_containers]: Check if containers already configured");
        if self.all_containers_crated() {
            self.dbg_print("[setup_containers]: Containers already configured.");
            return Ok(());
        }

        self.dbg_print("Setup API proxy container");
        self.setup_container_api_proxy()
            .await
            .expect("[TestEnv/CI:setup_containers]: Failed to setup API proxy container");

        self.dbg_print("Setup clickhouse container config");
        self.setup_container_clickhouse()
            .await
            .expect("[TestEnv/CI:setup_containers]: Failed to setup clickhouse container");

        self.dbg_print("Setup postgres container");
        self.setup_container_postgres_db()
            .await
            .expect("[TestEnv/CI:setup_containers]: Failed to setup SurrealDB container");

        self.dbg_print("Set all containers to created");
        self.set_all_containers_crated();

        Ok(())
    }
}
