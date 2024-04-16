use test_utils::prelude::DockerUtil;

pub fn get_docker_util() -> DockerUtil {
    DockerUtil::new().expect("Failed to create DockerUtil")
}

#[test]
fn test_stop_container() {
    let mut docker_util = get_docker_util();
    let result = docker_util.stop_container("test_container");

    assert!(result.is_ok())
}
