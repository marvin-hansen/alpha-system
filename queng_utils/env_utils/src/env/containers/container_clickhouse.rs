use crate::prelude::EnvironmentSetupError;
use crate::EnvUtil;
use specs_utils::prelude::clickhouse_container_specs;

impl EnvUtil {
    pub async fn setup_container_clickhouse(&mut self) -> Result<(), EnvironmentSetupError> {
        //
        self.dbg_print("Get docker util");
        let docker_util = self.docker_util();

        self.dbg_print("Setup api proxy container");
        let clickhouse_container_config = clickhouse_container_specs();
        let (clickhouse_container_name, clickhouse_container_port) = docker_util
            .setup_container(&clickhouse_container_config)
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "[TestEnv/CI:setup_clickhouse_container]: Failed to setup container: {}",
                    &clickhouse_container_config.container_name()
                )
            });

        self.dbg_print("Verify clickhouse container name and ports");
        assert_eq!(
            clickhouse_container_name,
            clickhouse_container_config.container_name()
        );
        assert_eq!(
            clickhouse_container_port,
            clickhouse_container_config.connection_port()
        );

        self.dbg_print("Set ClickHouse container names and ports");
        self.set_clickhouse_container_name(clickhouse_container_name);
        self.set_clickhouse_container_port(clickhouse_container_port);

        Ok(())
    }
}
