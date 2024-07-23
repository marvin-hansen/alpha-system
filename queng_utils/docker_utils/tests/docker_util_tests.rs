use container_specs::api_proxy_container_specs::api_proxy_container_config;
use container_specs::postgres_db_specs::postgres_db_container_config;
use docker_utils::DockerUtil;

#[tokio::test]
async fn test_pull() {
    let docker_util = DockerUtil::with_debug().expect("Failed to create DockerUtil");

    let container_config = api_proxy_container_config();
    let container_id = "apiproxy-7777";

    let image = container_config.image();
    let tag = container_config.tag();
    let image = &format!("{}:{}", image, tag);
    let platform = container_config.platform();

    let res = docker_util.pull_container_image(container_id, image, platform);
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_start_stop_container() {
    let docker_util = DockerUtil::with_debug().expect("Failed to create DockerUtil");

    let container_config = postgres_db_container_config();
    let container_id = "postgresdb-5432";

    let res = docker_util.get_or_start_container_config(&container_config);
    assert!(res.is_ok());

    let (container_name, container_port) = res.unwrap();
    assert_eq!(container_name, container_id);
    assert_eq!(container_port, 5432);

    let res = docker_util.stop_container(&container_id);

    assert!(res.is_ok());

    let res = docker_util.check_if_container_exists(&container_id);

    assert!(res.is_ok());
    assert!(!res.unwrap());
}
