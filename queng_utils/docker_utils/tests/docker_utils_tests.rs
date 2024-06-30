use container_specs::api_proxy_container_config::api_proxy_container_config;
use container_specs::clickhouse_container_config::clickhouse_container_config;
use std::time::Duration;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage, ImageExt,
};
use tokio::time::sleep;

#[tokio::test]
async fn test_clickhouse() {
    let container_config = clickhouse_container_config();
    let container_name = "test-clickhouse";
    // let api_proxy_container_config = api_proxy_container_config();

    let image = container_config.image();
    let tag = container_config.tag();
    let port = container_config.connection_port();

    let container = GenericImage::new(image, tag)
        .with_exposed_port(port.tcp())
        .with_wait_for(WaitFor::Duration {
            length: Duration::from_secs(1),
        })
        .with_container_name(container_name)
        .start()
        .await;

    assert!(container.is_ok());

    sleep(Duration::from_secs(5)).await;
}

#[tokio::test]
async fn test_api_proxy() {
    let container_config = api_proxy_container_config();
    let container_name = "test-api-proxy";

    let image = container_config.image();
    let tag = container_config.tag();
    let port = container_config.connection_port();

    println!("image: {}", image);
    println!("tag: {}", tag);
    println!("port: {}", port);

    let container = GenericImage::new(image, tag)
        .with_exposed_port(port.tcp())
        .with_wait_for(WaitFor::message_on_stdout(
            "Service on endpoint: 0.0.0.0:7777/",
        ))
        .with_container_name(container_name)
        .start()
        .await;

    assert!(container.is_ok());

    sleep(Duration::from_secs(5)).await;
}
