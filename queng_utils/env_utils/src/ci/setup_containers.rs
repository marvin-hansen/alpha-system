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
        //
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

    pub async fn setup_container_api_proxy(&mut self) -> Result<(), EnvironmentSetupError> {
        //
        self.dbg_print("Get docker util");
        let mut docker_util = self.docker_util();

        self.dbg_print("Setup api proxy container");
        let api_proxy_container_config = api_proxy_container_config();

        let (container_name, container_port) = self
            .setup_container(&api_proxy_container_config, &mut docker_util)
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "[TestEnv/CI:setup_api_proxy_container]: Failed to setup container: {}",
                    &api_proxy_container_config.container_name()
                )
            });

        self.dbg_print("Verify api proxy container name and ports");
        assert_eq!(container_name, api_proxy_container_config.container_name());
        assert_eq!(container_port, api_proxy_container_config.connection_port());

        self.dbg_print(&format!("OK container_name: {}", container_name));
        self.dbg_print(&format!("OK container_port: {}", container_port));

        self.dbg_print("Set api proxy container name and ports");
        self.set_api_proxy_container_name(container_name);
        self.set_api_proxy_container_port(container_port);

        Ok(())
    }

    pub async fn setup_container_clickhouse(&mut self) -> Result<(), EnvironmentSetupError> {
        //
        self.dbg_print("Get docker util");
        let mut docker_util = self.docker_util();

        self.dbg_print("Setup api proxy container");
        let clickhouse_container_config = clickhouse_container_config();
        let (clickhouse_container_name, clickhouse_container_port) = self
            .setup_container(&clickhouse_container_config, &mut docker_util)
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

    async fn setup_container(
        &self,
        container_config: &ContainerConfig<'_>,
        docker_util: &mut DockerUtil,
    ) -> Result<(String, u16), EnvironmentSetupError> {
        //
        let container_name = &container_config.container_name();
        let container_port = &container_config.connection_port();
        let wait_duration = container_config.wait_duration();
        let target_tag = container_config.tag();

        self.dbg_print(&format!(
            "Check if Container already exists: {}",
            container_name
        ));

        let exists = docker_util
            .check_if_container_exists(container_name)
            .expect(&format!(
                "[get_running_container]:  container already exists: {}",
                container_name
            ));
        self.dbg_print(&format!("Container {} exists: {}", container_name, exists));

        // If the container already exists, check if its using the current target tag from the config.
        // This corrects config drift in case the container config got updated with a newer or different tag.
        if exists {
            self.dbg_print(&format!(
                "Check if running Container {} uses target tag: {}",
                container_name, target_tag,
            ));

            let container_current = docker_util
                .check_if_running_container_uses_target_tag(container_name, target_tag)
                .expect(&format!(
                    "[TestEnv/CI:setup_container]: Failed to check if container {} use target tag: {}",
                    container_name, target_tag,
                ));

            if !container_current {
                self.dbg_print(&format!(
                    "Container uses DIFFERENT tag : {}",
                    container_name
                ));
                self.dbg_print(&format!("STOP running Container : {}", container_name));

                docker_util.stop_container(container_name).expect(&format!(
                    "[TestEnv/CI:setup_container]: Failed to check stop container {} ",
                    container_name,
                ))
            } else {
                self.dbg_print(&format!(
                    "Container {} uses target tag: {}",
                    container_name, container_current
                ));
            }
        }

        return if !exists {
            self.dbg_print(&format!(
                "Start container {} with target tag {}",
                container_name, target_tag
            ));

            let (container_name, container_port) = docker_util
                .get_or_start_container_config(container_config)
                .unwrap_or_else(|_| {
                    panic!(
                        "[TestEnv/CI:setup_container]: Failed to setup container: {}",
                        container_name
                    )
                });

            self.dbg_print(&format!(
                "Wait {} seconds for {} container to complete setup & finish boot sequence",
                wait_duration, &container_name
            ));
            sleep(Duration::from_secs(wait_duration)).await;

            Ok((container_name, container_port))
        } else {
            self.dbg_print(&format!(
                "Reuse Container {} with target tag {}",
                container_name, target_tag
            ));

            Ok((container_name.to_owned(), container_port.to_owned()))
        };
    }
}
