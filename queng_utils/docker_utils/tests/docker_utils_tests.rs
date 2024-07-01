use container_specs::api_proxy_container_config::api_proxy_container_config;
use container_specs::clickhouse_container_config::clickhouse_container_config;

#[tokio::test]
async fn test_clickhouse() {
    let container_config = clickhouse_container_config();
    // let container_name = "test-clickhouse";

    let image = container_config.image();
    let tag = container_config.tag();
    let port = container_config.connection_port();

    println!("image: {}", image);
    println!("tag: {}", tag);
    println!("port: {}", port);
}

#[tokio::test]
async fn test_api_proxy() {
    let container_config = api_proxy_container_config();
    // let container_name = "test-api-proxy";

    let image = container_config.image();
    let tag = container_config.tag();
    let port = container_config.connection_port();

    println!("image: {}", image);
    println!("tag: {}", tag);
    println!("port: {}", port);
}
