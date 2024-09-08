use crate::docker::DockerUtil;
use crate::error::DockerError;
use container_specs::clickhouse_container_specs::clickhouse_container_config;

impl DockerUtil {
    pub async fn setup_container_clickhouse(&self) -> Result<(), DockerError> {
        self.dbg_print("Setup api proxy container");
        let clickhouse_container_config = clickhouse_container_config();
        let (clickhouse_container_name, clickhouse_container_port) = self
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
        self.dbg_print(&format!("OK container_name: {}", clickhouse_container_name));
        self.dbg_print(&format!("OK container_port: {}", clickhouse_container_port));

        Ok(())
    }
}
