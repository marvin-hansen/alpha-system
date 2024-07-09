use crate::prelude::EnvironmentSetupError;
use crate::EnvUtil;
impl EnvUtil {
    pub async fn setup_containers(&mut self) -> Result<(), EnvironmentSetupError> {
        //
        self.dbg_print("[setup_containers]: Check if containers already configured");
        if self.containers_crated {
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

        self.dbg_print("Set containers to created");
        self.set_containers_crated();

        Ok(())
    }
}
