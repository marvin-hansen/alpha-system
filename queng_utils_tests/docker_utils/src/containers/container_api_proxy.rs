use crate::docker::DockerUtil;
use crate::error::DockerError;
use container_specs::api_proxy_container_specs::api_proxy_container_config;

impl DockerUtil {
    pub async fn setup_container_api_proxy(&self) -> Result<(), DockerError> {
        //
        self.dbg_print("Setup api proxy container");
        let container_config = api_proxy_container_config();

        let (container_name, container_port) = self
            .setup_container(&container_config)
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "[TestEnv/CI:setup_api_proxy_container]: Failed to setup container: {}",
                    &container_config.container_name()
                )
            });

        self.dbg_print("Verify api proxy container name and ports");
        assert_eq!(container_name, container_config.container_name());
        assert_eq!(container_port, container_config.connection_port());

        self.dbg_print(&format!("OK container_name: {}", container_name));
        self.dbg_print(&format!("OK container_port: {}", container_port));

        Ok(())
    }
}
