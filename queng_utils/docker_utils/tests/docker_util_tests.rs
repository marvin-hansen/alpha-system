use docker_utils::DockerUtil;
use std::{thread, time};

#[tokio::test]
async fn docker_env_util_setup_ci() {
    // Create a new DockerUtil in debug mode. Without debug, just call new()
    let mut docker_util = DockerUtil::with_debug().expect("Failed to create DockerUtil");
    // Set default wait duration
    let wait_time = time::Duration::from_millis(500);

    println!();
    println!(">> Test get_or_start_container: Create a new container");
    println!();

    let name = "clickhouse";
    let connection_port = 9000;
    let additional_ports = &[8123];
    let image = "clickhouse/clickhouse-server:24.3.2";
    let reuse_container = false;

    let result = docker_util.get_or_start_container(
        name,
        image,
        connection_port,
        additional_ports,
        reuse_container,
    );
    if result.is_err() {
        println!("{}", result.as_ref().unwrap_err());
    }

    assert!(result.is_ok());

    let (container_name, port) = result.unwrap();
    assert_eq!(container_name, "clickhouse-9000");
    assert_eq!(port, 9000);

    let exists = docker_util
        .check_if_container_exists(&container_name)
        .expect("Failed to check if container exists");
    assert!(exists);

    println!(
        "✅ OK: Container name: {} and port: {}",
        container_name, port
    );
    println!("✅ OK: Container created");
    println!();

    // Pause execution to check Docker UI/CLI if the container is up & running
    thread::sleep(wait_time);

    println!(">> Test get_or_start_container: Re-use an existing container");
    println!();

    let reuse_container = true;

    let result = docker_util.get_or_start_container(
        name,
        image,
        connection_port,
        additional_ports,
        reuse_container,
    );
    if result.is_err() {
        println!("{}", result.as_ref().unwrap_err());
    }

    assert!(result.is_ok());

    let (container_name, port) = result.unwrap();
    assert_eq!(container_name, "clickhouse-9000");
    assert_eq!(port, 9000);

    let exists = docker_util
        .check_if_container_exists(&container_name)
        .expect("Failed to check if container exists");
    assert!(exists);

    println!(
        "✅ OK: Container name: {} and port: {}",
        container_name, port
    );
    println!("✅ OK: Container re-used");
    println!();

    // Pause execution to check Docker UI/CLI if the container is up & running
    thread::sleep(wait_time);

    println!(">> Test get_or_start_container: Stop container");
    println!();

    let result = docker_util.stop_container(&container_name);
    if result.is_err() {
        println!("{}", result.as_ref().unwrap_err());
    }

    assert!(result.is_ok());

    let exists = docker_util
        .check_if_container_exists(&container_name)
        .expect("Failed to check if container exists");
    assert!(!exists);

    println!("✅ OK: Container stopped");
    println!();

    println!("All tests passed:");
    println!("  ✅ OK: Container created");
    println!("  ✅ OK: Container re-used");
    println!("  ✅ OK: Container stopped");
}
