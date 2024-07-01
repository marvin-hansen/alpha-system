use container_specs::api_proxy_container_config::api_proxy_container_config;
use container_specs::clickhouse_container_config::clickhouse_container_config;
use docker_utils::DockerUtil;

#[tokio::test]
async fn test_clickhouse() {
    let docker_util = DockerUtil::with_debug().expect("Failed to create DockerUtil");

    let container_config = clickhouse_container_config();
    let container_id = "test-clickhouse-9000";

    let image = container_config.image();
    let tag = container_config.tag();
    let port = container_config.connection_port();
    let image = &format!("{}:{}", image, tag);
    let platform = container_config.platform();

    println!("image: {}", image);
    println!("tag: {}", tag);
    println!("port: {}", port);
    println!("platform: {:?}", platform);

    let res = docker_util.pull_container_image(container_id, &image, platform);
    assert!(res.is_ok());

    let res = docker_util.pull_container_image(container_id, image, platform);
    assert!(res.is_ok());

    let res = docker_util.start_container(container_id, port, None, platform, image);

    assert!(res.is_ok());
    let (container_id, _) = res.unwrap();

    let res = docker_util.stop_container(&container_id);
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_api_proxy() {
    let docker_util = DockerUtil::with_debug().expect("Failed to create DockerUtil");

    let container_config = api_proxy_container_config();
    let container_id = "test-api-proxy-7777";

    let image = container_config.image();
    let tag = container_config.tag();
    let port = container_config.connection_port();
    let platform = container_config.platform();
    let image = &format!("{}:{}", image, tag);

    println!("image: {}", image);
    println!("tag: {}", tag);
    println!("port: {}", port);
    println!("platform: {:?}", platform);

    let res = docker_util.start_container(container_id, port, None, platform, image);

    assert!(res.is_ok());
    let (container_id, _) = res.unwrap();

    let res = docker_util.stop_container(&container_id);
    assert!(res.is_ok());
}
