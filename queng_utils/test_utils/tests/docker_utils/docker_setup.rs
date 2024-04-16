use crate::docker_utils::docker_shared;
use test_utils::prelude::DockerUtil;

#[test]
fn test_new() {
    let result = DockerUtil::new();
    assert!(result.is_ok());
}

#[test]
fn test_start_container_no_reuse() {
    let mut docker_util = docker_shared::get_docker_util();
    let result = docker_util.start_container(
        "test_container",
        "alpine:latest",
        Some(vec!["/bin/true".to_string()]),
        7070,
        false,
    );

    assert!(result.is_ok())
}

#[test]
fn test_start_container_reuse_running_container() {
    let mut docker_util = docker_shared::get_docker_util();
    let result = docker_util.start_container(
        "test_container",
        "test_image",
        Some(vec!["/bin/true".to_string()]),
        7070,
        true,
    );

    assert!(result.is_ok())
}
