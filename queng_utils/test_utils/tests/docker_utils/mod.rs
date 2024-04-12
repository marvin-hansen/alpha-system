use test_utils::prelude::DockerUtil;

#[test]
fn test_is_container_running() {
    let docker_util = DockerUtil::new();
    let result = docker_util.is_container_running("test_container");

    assert!(result.is_ok());
}

#[test]
fn test_get_running_container() {
    let docker_util = DockerUtil::new();
    let running_container = docker_util.get_running_container();

    assert!(running_container.is_ok())
}

#[test]
fn test_start_container() {
    let docker_util = DockerUtil::new();
    let result = docker_util.start_container("test_container", "test_image", 8080, false);

    assert!(result.is_ok())
}

#[test]
fn test_stop_container() {
    let docker_util = DockerUtil::new();
    let result = docker_util.stop_container("test_container");

    assert!(result.is_ok())
}
