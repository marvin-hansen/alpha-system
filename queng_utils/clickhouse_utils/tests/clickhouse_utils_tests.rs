use clickhouse_utils::ClickhouseUtil;
use container_specs::clickhouse_container_config::clickhouse_container_config;
use docker_utils::DockerUtil;
use std::{thread, time};

#[tokio::test]
async fn docker_env_util_setup_db_test() {
    // Create a new DockerUtil in debug mode. Without debug, just call new()
    let docker_util = DockerUtil::with_debug().expect("Failed to create DockerUtil");
    // Set default wait duration
    let wait_time = time::Duration::from_millis(500);

    println!();
    println!(">> Test get_or_start_container: Create a new container");
    println!();

    // Create a new container from config and start or reuse it
    let container_config = clickhouse_container_config();
    let result = docker_util.get_or_start_container_config(&container_config);
    if result.is_err() {
        println!("{}", result.as_ref().unwrap_err());
    }

    // Pause execution to check Docker UI/CLI if the container is up & running
    thread::sleep(wait_time);

    assert!(result.is_ok());

    let (container_name, port) = result.unwrap();
    assert_eq!(container_name, "clickhouse-9000");
    assert_eq!(port, 9000);

    let exists = docker_util
        .check_if_container_exists(&container_name)
        .expect("Failed to check if container exists");
    assert!(exists);

    let dsn = "127.0.0.1:9000".to_string();
    let result = ClickhouseUtil::new(dsn).await;

    assert!(result.is_ok());

    let ch_utils = result.unwrap();

    let setup_result = ch_utils.setup_db().await;
    assert!(setup_result.is_ok());

    thread::sleep(wait_time);

    let teardown_result = ch_utils.teardown_db().await;
    assert!(teardown_result.is_ok());
}
