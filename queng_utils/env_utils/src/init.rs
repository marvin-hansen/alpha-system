use crate::prelude::EnvironmentSetupError;
use crate::EnvUtil;
use common::prelude::ContainerConfig;
use docker_utils::error::DockerError;
use docker_utils::DockerUtil;
use kaiko_utils::{KaikoUtil, KaikoUtilError};

impl EnvUtil {
    pub(super) fn init_docker_util(dbg: bool) -> Result<DockerUtil, DockerError> {
        if dbg {
            return match DockerUtil::with_debug() {
                Ok(docker_util) => Ok(docker_util),
                Err(e) => Err(e),
            };
        }

        match DockerUtil::new() {
            Ok(docker_util) => Ok(docker_util),
            Err(e) => Err(e),
        }
    }

    pub(super) async fn init_kaiko_util(dbg: bool) -> Result<KaikoUtil, KaikoUtilError> {
        if dbg {
            KaikoUtil::with_debug()
        } else {
            KaikoUtil::new()
        }
    }

    pub(super) fn init_container(
        container_config: &ContainerConfig<'_>,
        docker_util: &DockerUtil,
    ) -> Result<(String, u16, bool), EnvironmentSetupError> {
        //
        let container_name = container_config.container_name();

        let exists = docker_util
            .check_if_container_exists(&container_name)
            .unwrap_or_else(|_| {
                panic!(
                    "[TestEnv/init_container]: Failed to check if container exists: {}",
                    &container_name
                )
            });

        if exists {
            let container_name = container_config.container_name();
            let container_port = container_config.connection_port();

            Ok((container_name, container_port, true))
        } else {
            Ok((String::from("CONTAINER NOT INITIALIZED"), 0, false))
        }
    }
}
