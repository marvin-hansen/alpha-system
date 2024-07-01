use container_specs::clickhouse_container_config::clickhouse_container_config;
use docker_utils::DockerUtil;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_pull_container() {
    let docker_util = DockerUtil::with_debug().expect("Failed to create DockerUtil");

    let container_config = clickhouse_container_config();
    let container_id = "test-clickhouse-9000";

    let image = container_config.image();
    let tag = container_config.tag();
    let image = &format!("{}:{}", image, tag);
    let platform = container_config.platform();

    let res = docker_util.pull_container_image(container_id, &image, platform);
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_start_stop_container() {
    let docker_util = DockerUtil::with_debug().expect("Failed to create DockerUtil");
    let container_config = clickhouse_container_config();
    let container_id = "test-clickhouse-9000";

    let image = container_config.image();
    let tag = container_config.tag();
    let port = container_config.connection_port();
    let image = &format!("{}:{}", image, tag);
    let platform = container_config.platform();

    let res = docker_util.start_container(container_id, port, None, platform, image);
    assert!(res.is_ok());

    sleep(Duration::from_secs(3)).await;

    let res = docker_util.stop_container(&container_id);
    assert!(res.is_ok());
}
