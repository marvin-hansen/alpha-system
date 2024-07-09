use crate::prelude::EnvironmentSetupError;
use crate::EnvUtil;
use container_specs::api_proxy_container_config::api_proxy_container_config;

impl EnvUtil {
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
}
