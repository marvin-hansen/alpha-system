use crate::prelude::EnvironmentSetupError;
use crate::EnvUtil;
use common::prelude::ContainerConfig;
use docker_utils::DockerUtil;
use std::time::Duration;
use tokio::time::sleep;

impl EnvUtil {
    /// Sets up a container for the environment.
    ///
    /// This method sets up a container for the environment by performing the following steps:
    ///
    /// 1. Retrieves the `DockerUtil` object.
    ///
    /// 2. Retrieves the container configuration from the environment configuration.
    ///
    /// 3. Starts the container using the `DockerUtil` object.
    ///
    /// 4. Waits for the container to be ready by checking its logs.
    ///
    /// # Arguments
    ///
    /// * `docker_util` - A reference to a `DockerUtil` object.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the container is set up successfully, or an `Err` variant of `EnvironmentSetupError` if an error occurs during the setup process.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `EnvironmentSetupError` if any of the following errors occur during the setup process:
    ///
    /// - `ContainerStartError`: If there is an error starting the container.
    /// - `ContainerLogsError`: If there is an error retrieving the container logs.
    /// - `ContainerNotReadyError`: If the container is not ready within the specified timeout period.
    ///
    pub(crate) async fn setup_container(
        &self,
        container_config: &ContainerConfig<'_>,
        docker_util: &mut DockerUtil,
    ) -> Result<(String, u16), EnvironmentSetupError> {
        //
        let container_name = &container_config.container_name();
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

        let (container_name, container_port) = docker_util
            .get_or_start_container_config(container_config)
            .unwrap_or_else(|_| {
                panic!(
                    "[TestEnv/CI:setup_container]: Failed to setup container: {}",
                    container_name
                )
            });

        return if !exists {
            self.dbg_print(&format!(
                "Start container {} with target tag {}",
                container_name, target_tag
            ));

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

            Ok((container_name, container_port))
        };
    }
}
