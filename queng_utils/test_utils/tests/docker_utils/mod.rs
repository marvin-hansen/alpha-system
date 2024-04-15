use test_utils::prelude::DockerUtil;

fn get_docker_util() -> DockerUtil {
    DockerUtil::new().expect("Failed to create DockerUtil")
}

#[test]
fn test_new() {
    let result = DockerUtil::new();
    assert!(result.is_ok());
}

#[test]
fn test_start_container_no_reuse() {
    let mut docker_util = get_docker_util();
    let result = docker_util.start_container("test_container", "test_image", 7070, false);

    assert!(result.is_ok())
}

#[test]
fn test_start_container_reuse_running_container() {
    let mut docker_util = get_docker_util();
    let result = docker_util.start_container("test_container", "test_image", 7070, false);

    assert!(result.is_ok())
}

#[test]
fn test_get_container_status() {
    let mut docker_util = get_docker_util();
    let result = docker_util.check_if_container_exists("test_container");

    assert!(result.is_ok());
}

#[test]
fn test_get_running_container() {
    let docker_util = get_docker_util();
    let running_container = docker_util.get_running_container();

    assert!(running_container.is_ok())
}

#[test]
fn test_stop_container() {
    let mut docker_util = get_docker_util();
    let result = docker_util.stop_container("test_container");

    assert!(result.is_ok())
}
