use container_specs::clickhouse_container_config::clickhouse_container_config;
use docker_utils::DockerUtil;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_start_stop_container() {
    let docker_util = DockerUtil::with_debug().expect("Failed to create DockerUtil");
    let container_config = clickhouse_container_config();
    let container_id = "test-clickhouse-9000";

    let res = docker_util.get_or_start_container_config(&container_config);
    assert!(res.is_ok());

    sleep(Duration::from_secs(3)).await;

    let res = docker_util.stop_container(&container_id);
    assert!(res.is_ok());
}
