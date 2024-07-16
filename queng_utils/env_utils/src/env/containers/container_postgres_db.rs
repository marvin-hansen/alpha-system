use crate::prelude::EnvironmentSetupError;
use specs_utils::prelude::postgres_db_container_specs;

use crate::EnvUtil;

impl EnvUtil {
    pub async fn setup_container_postgres_db(&mut self) -> Result<(), EnvironmentSetupError> {
        //
        self.dbg_print("Get docker util");
        let mut docker_util = self.docker_util();

        self.dbg_print("Setup Postgres DB container");

        let container_config = postgres_db_container_specs();

        let (container_name, container_port) = self
            .setup_container(&container_config, &mut docker_util)
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "[TestEnv/CI:setup_api_proxy_container]: Failed to setup container: {}",
                    &container_config.container_name()
                )
            });

        self.dbg_print("Verify Postgres container name and ports");
        assert_eq!(container_name, container_config.container_name());
        assert_eq!(container_port, container_config.connection_port());

        self.dbg_print(&format!("OK container_name: {}", container_name));
        self.dbg_print(&format!("OK container_port: {}", container_port));

        self.dbg_print("Set api SurrealDB container name and ports");
        self.set_postgres_db_container_name(container_name);
        self.set_postgres_db_container_port(container_port);

        Ok(())
    }
}
