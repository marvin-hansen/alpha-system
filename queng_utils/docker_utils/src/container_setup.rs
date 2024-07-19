use crate::error::DockerError;
use crate::DockerUtil;
use common_config::prelude::ContainerConfig;
use std::time::Duration;
use tokio::time::sleep;
impl DockerUtil {
    /// Sets up a Docker container according to its configuration
    ///
    ///
    /// # Arguments
    ///
    /// * `container_config` - A reference to a `ContainerConfig` object.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the container is set up successfully, or an `Err` variant of `EnvironmentSetupError` if an error occurs during the setup process.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `DockerError` if any of the following errors occur during the setup process:
    ///
    pub async fn setup_container(
        &self,
        container_config: &ContainerConfig<'_>,
    ) -> Result<(String, u16), DockerError> {
        let container_name = &container_config.container_name();
        let wait_duration = container_config.wait_duration();
        let target_tag = container_config.tag();

        self.dbg_print(&format!(
            "Check if Container already exists: {}",
            container_name
        ));

        let exists = self
            .check_if_container_exists(container_name)
            .unwrap_or_else(|_| {
                panic!(
                    "[get_running_container]:  container already exists: {}",
                    container_name
                )
            });
        self.dbg_print(&format!("Container {} exists: {}", container_name, exists));

        if exists {
            self.dbg_print(&format!(
                "Check if running Container {} uses target tag: {}",
                container_name, target_tag,
            ));

            let container_current = self
                .check_if_running_container_uses_target_tag(container_name, target_tag)
                .unwrap_or_else(|_| panic!("[TestEnv/CI:setup_container]: Failed to check if container {} use target tag: {}",
                                           container_name, target_tag));

            if !container_current {
                self.dbg_print(&format!(
                    "Container uses DIFFERENT tag : {}",
                    container_name
                ));
                self.dbg_print(&format!("STOP running Container : {}", container_name));

                self.stop_container(container_name).unwrap_or_else(|_| {
                    panic!(
                        "[TestEnv/CI:setup_container]: Failed to check stop container {} ",
                        container_name
                    )
                })
            } else {
                self.dbg_print(&format!(
                    "Container {} uses target tag: {}",
                    container_name, container_current
                ));
            }
        }

        let (container_name, container_port) = self
            .get_or_start_container_config(container_config)
            .unwrap_or_else(|_| {
                panic!(
                    "[TestEnv/CI:setup_container]: Failed to setup container: {}",
                    container_name
                )
            });

        if !exists {
            self.dbg_print(&format!(
                "Start container {} with target tag {}",
                container_name, target_tag
            ));

            self.dbg_print(&format!(
                "Wait {} seconds for {} container to complete setup & finish boot sequence",
                wait_duration, &container_name
            ));
            sleep(Duration::from_secs(wait_duration)).await;
        } else {
            self.dbg_print(&format!(
                "Reuse Container {} with target tag {}",
                container_name, target_tag
            ));
        }

        Ok((container_name, container_port))
    }
}
