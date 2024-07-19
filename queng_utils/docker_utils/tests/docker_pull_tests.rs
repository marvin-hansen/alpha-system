use container_specs::api_proxy_container_specs::api_proxy_container_config;
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
