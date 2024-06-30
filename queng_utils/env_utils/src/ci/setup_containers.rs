use crate::prelude::EnvironmentSetupError;
use crate::EnvUtil;
use common::prelude::ContainerConfig;
use container_specs::api_proxy_container_config::api_proxy_container_config;
use container_specs::clickhouse_container_config::clickhouse_container_config;
use docker_utils::DockerUtil;
use std::time::Duration;
use tokio::time::sleep;

impl EnvUtil {
    pub async fn setup_containers(&mut self) -> Result<(), EnvironmentSetupError> {
        self.dbg_print("Get docker util");
        let mut docker_util = self.docker_util();

        self.dbg_print("Setup API proxy container");
        self.setup_api_proxy_container(&mut docker_util)
            .await
            .expect("[TestEnv/CI:setup_containers]: Failed to setup API proxy container");

        self.dbg_print("Setup clickhouse container config");
        self.setup_clickhouse_container(&mut docker_util)
            .await
            .expect("[TestEnv/CI:setup_containers]: Failed to setup clickhouse container");

        self.dbg_print("Set containers to created");
        self.set_containers_crated();

        Ok(())
    }

    pub async fn setup_clickhouse_container(
        &mut self,
        docker_util: &mut DockerUtil,
    ) -> Result<(), EnvironmentSetupError> {
        //
        self.dbg_print("Setup api proxy container");
        let clickhouse_container_config = clickhouse_container_config();
        let (clickhouse_container_name, clickhouse_container_port) = self
            .setup_container(&clickhouse_container_config, docker_util)
            .await
            .expect(
                format!(
                    "[TestEnv/CI:setup_clickhouse_container]: Failed to setup container: {}",
                    &clickhouse_container_config.container_name()
                )
                .as_str(),
            );

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

    pub async fn setup_api_proxy_container(
        &mut self,
        docker_util: &mut DockerUtil,
    ) -> Result<(), EnvironmentSetupError> {
        //
        self.dbg_print("Setup api proxy container");
        let api_proxy_container_config = api_proxy_container_config();

        let (container_name, container_port) = self
            .setup_container(&api_proxy_container_config, docker_util)
            .await
            .expect(
                format!(
                    "[TestEnv/CI:setup_api_proxy_container]: Failed to setup container: {}",
                    &api_proxy_container_config.container_name()
                )
                .as_str(),
            );

        self.dbg_print("Verify api proxy container name and ports");
        assert_eq!(container_name, api_proxy_container_config.container_name());
        assert_eq!(container_port, api_proxy_container_config.connection_port());

        self.dbg_print(&format!("OK: {}", container_name));
        self.dbg_print(&format!("OK: {}", container_port));

        self.dbg_print("Set api proxy container name and ports");
        self.set_api_proxy_container_name(container_name);
        self.set_api_proxy_container_port(container_port);

        Ok(())
    }

    async fn setup_container(
        &mut self,
        container_config: &ContainerConfig<'_>,
        docker_util: &mut DockerUtil,
    ) -> Result<(String, u16), EnvironmentSetupError> {
        //
        let container_name = container_config.container_name();
        let wait_duration = container_config.wait_duration();

        self.dbg_print(&format!("Check if container exists: {}", &container_name));
        let exists = docker_util
            .check_if_container_exists(&container_name)
            .expect(
                format!(
                    "[TestEnv/CI:setup_container]: Failed to check if container exists: {}",
                    &container_name
                )
                .as_str(),
            );

        if exists {
            let container_name = container_config.container_name();
            let container_port = container_config.connection_port();

            self.dbg_print(&format!(
                "OK: Container name: {} already exists",
                &container_name
            ));
            return Ok((container_name, container_port));
        }

        self.dbg_print(&format!("NO: Container NOT found: {} ", &container_name));

        self.dbg_print(&format!("Setup Container: {}", &container_name));
        let (container_name, container_port) = docker_util
            .get_or_start_container_config(&container_config)
            .expect(
                format!(
                    "[TestEnv/CI:setup_container]: Failed to setup container: {}",
                    &container_name
                )
                .as_str(),
            );

        self.dbg_print(&format!(
            "Wait {} seconds for {} container to complete setup & finish boot sequence",
            wait_duration, &container_name
        ));
        sleep(Duration::from_secs(wait_duration)).await;

        Ok((container_name, container_port))
    }
}
