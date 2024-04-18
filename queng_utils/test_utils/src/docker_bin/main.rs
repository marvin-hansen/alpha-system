use std::{thread, time};
use test_utils::prelude::DockerUtil;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new DockerUtil in debug mode. Without debug, just call new()
    let mut docker_util = DockerUtil::with_debug().expect("Failed to create DockerUtil");
    // Set default wait duration
    let five_seconds = time::Duration::from_secs_f32(5.0f32);

    println!();
    println!(">> Test get_or_start_container: Create a new container");
    println!();

    let name = "nginx";
    let port = 80;
    let image = "nginx:latest";
    let reuse_container = false;

    let result = docker_util.get_or_start_container(name, image, port, reuse_container);
    if result.is_err() {
        println!("{}", result.as_ref().unwrap_err());
    }

    assert!(result.is_ok());

    let (container_name, port) = result.unwrap();
    assert_eq!(container_name, "nginx-80");
    assert_eq!(port, 80);

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
    thread::sleep(five_seconds);

    println!(">> Test get_or_start_container: Re-use an existing container");
    println!();

    let reuse_container = true;

    let result = docker_util.get_or_start_container(name, image, port, reuse_container);
    if result.is_err() {
        println!("{}", result.as_ref().unwrap_err());
    }

    assert!(result.is_ok());

    let (container_name, port) = result.unwrap();
    assert_eq!(container_name, "nginx-80");
    assert_eq!(port, 80);

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
    thread::sleep(five_seconds);

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

    Ok(())
}
