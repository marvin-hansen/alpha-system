use test_utils::prelude::DockerUtil;

fn get_docker_util() -> DockerUtil {
    DockerUtil::new().expect("Failed to create DockerUtil")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut docker_util = get_docker_util();

    println!("Test get_or_start_container");
    let name = "nginx";
    let port = 80;
    let image = "nginx:latest";
    let reuse_container = true;

    let result = docker_util.get_or_start_container(name, image, port, reuse_container);

    if result.is_err() {
        println!("{}", result.as_ref().unwrap_err());
    }
    assert!(result.is_ok());
    let (container_name, port) = result.unwrap();

    println!("Container name: {} and port: {}", container_name, port);

    Ok(())
}
