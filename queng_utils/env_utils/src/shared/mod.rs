use crate::prelude::EnvUtil;
use clickhouse_utils::{ClickHouseClient, ClickhouseUtil};
use docker_utils::container_config::ContainerConfig;
use docker_utils::docker_error::DockerError;
use docker_utils::DockerUtil;

impl EnvUtil {
    pub fn get_docker_util(&self) -> Result<DockerUtil, DockerError> {
        if self.dbg {
            return match DockerUtil::with_debug() {
                Ok(docker_util) => Ok(docker_util),
                Err(e) => Err(e),
            };
        }

        return match DockerUtil::new() {
            Ok(docker_util) => Ok(docker_util),
            Err(e) => Err(e),
        };
    }

    pub(crate) async fn get_clickhouse_util(&self, client: ClickHouseClient) -> ClickhouseUtil {
        return if self.dbg {
            ClickhouseUtil::from_client_with_debug(client)
        } else {
            ClickhouseUtil::from_client(client)
        };
    }

    pub(crate) async fn get_clickhouse_client(
        &self,
        container_config: &ContainerConfig<'_>,
    ) -> ClickHouseClient {
        // DB connection string
        let dsn = format!(
            "{}:{}",
            container_config.url(),
            container_config.connection_port(),
        );

        // Get clickhouse client.
        ClickhouseUtil::get_clickhouse_client(dsn).await
    }
}
