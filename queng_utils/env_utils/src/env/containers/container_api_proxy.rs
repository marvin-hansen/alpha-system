use crate::prelude::EnvironmentSetupError;
use crate::EnvUtil;
use specs_utils::prelude::api_proxy_container_specs;

impl EnvUtil {
    pub async fn setup_container_api_proxy(&mut self) -> Result<(), EnvironmentSetupError> {
        //
        self.dbg_print("Get docker util");
        let docker_util = self.docker_util();

        self.dbg_print("Setup api proxy container");
        let container_config = api_proxy_container_specs();

        let (container_name, container_port) = docker_util
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

        self.dbg_print("Set api proxy container name and ports");
        self.set_api_proxy_container_name(container_name);
        self.set_api_proxy_container_port(container_port);

        Ok(())
    }
}
