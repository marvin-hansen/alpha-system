use crate::prelude::EnvironmentError;
use crate::EnvUtil;
use container_specs::api_proxy_container_config::api_proxy_container_config;
use container_specs::clickhouse_container_config::clickhouse_container_config;
use std::thread::sleep;
use std::time::Duration;

impl EnvUtil {
    pub async fn setup_containers(&mut self) -> Result<(), EnvironmentError> {
        self.dbg_print("Get docker util");
        let mut docker_util = self
            .get_docker_util()
            .expect("[TestEnv:CI]: Failed to get docker util");

        self.dbg_print("Get api proxy container config");
        let api_proxy_container_config = api_proxy_container_config();

        self.dbg_print("Get or reuse api proxy container");
        let (api_proxy_container_name, api_proxy_container_port) = docker_util
            .get_or_start_container_config(&api_proxy_container_config)
            .expect("[TestEnv:CI]: Failed to get or reuse clickhouse container");

        // Give the api proxy container some extra time to complete booting up.
        sleep(Duration::from_secs(90));

        assert_eq!(api_proxy_container_name, "apiproxy-7777");
        assert_eq!(api_proxy_container_port, 7777);

        self.dbg_print("Set apiproxy container names and ports");
        // API Proxy
        self.set_api_proxy_container_name(api_proxy_container_name);
        self.set_api_proxy_container_port(api_proxy_container_port);

        self.dbg_print("Get clickhouse container config");
        let clickhouse_container_config = clickhouse_container_config();

        self.dbg_print("Get or reuse clickhouse container");
        let (clickhouse_container_name, clickhouse_container_port) = docker_util
            .get_or_start_container_config(&clickhouse_container_config)
            .expect("[TestEnv:CI]: Failed to get or reuse clickhouse container");

        // Give the container some extra time to complete booting up.
        // Otherwise, you may get a connection refused error when connecting the client.
        sleep(Duration::from_secs(10));

        assert_eq!(clickhouse_container_name, "clickhouse-9000");
        assert_eq!(clickhouse_container_port, 9000);

        self.dbg_print("Set ClickHouse container names and ports");
        // ClickHouse
        self.set_clickhouse_container_name(clickhouse_container_name);
        self.set_clickhouse_container_port(clickhouse_container_port);

        Ok(())
    }
}
