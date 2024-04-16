use test_utils::prelude::DockerUtil;

pub fn get_docker_util() -> DockerUtil {
    DockerUtil::new().expect("Failed to create DockerUtil")
}

#[test]
fn test_get_container_status() {
    let mut docker_util = get_docker_util();
    let result = docker_util.check_if_container_exists("test_container");

    assert!(result.is_ok());

    let exists = result.expect("Failed to check if container exists");

    assert!(exists)
}

#[test]
fn test_get_running_container() {
    let docker_util = get_docker_util();
    let running_container = docker_util.get_running_container();

    assert!(running_container.is_ok())
}
