use crate::prelude::EnvUtil;
use clickhouse_utils::{ClickHouseClient, ClickhouseUtil};
use common::prelude::ContainerConfig;
use docker_utils::error::DockerError;
use docker_utils::DockerUtil;
use kaiko_utils::{KaikoUtil, KaikoUtilError};

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

    pub(crate) async fn get_kaiko_util(&self) -> Result<KaikoUtil, KaikoUtilError> {
        return if self.dbg {
            KaikoUtil::with_debug()
        } else {
            KaikoUtil::new()
        };
    }
}
