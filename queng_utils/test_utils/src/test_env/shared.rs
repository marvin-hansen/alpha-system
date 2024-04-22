use crate::docker_util::DockerUtil;
use crate::prelude::{ContainerConfig, DockerError};
use db_utils::prelude::ClickHouseClient;

pub(crate) async fn get_clickhouse_client(
    container_config: &ContainerConfig<'_>,
) -> ClickHouseClient {
    // DB connection string
    let dsn = format!("{}:{}", container_config.url(), container_config.port(),);

    // Get clickhouse client.
    db_utils::get_clickhouse_client(dsn).await
}

pub(crate) fn get_docker_util() -> Result<DockerUtil, DockerError> {
    return match DockerUtil::new() {
        Ok(docker_util) => Ok(docker_util),
        Err(e) => Err(e),
    };
}
