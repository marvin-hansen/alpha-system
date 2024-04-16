use crate::docker_utils::docker_shared;

#[test]
fn test_stop_container() {
    let mut docker_util = docker_shared::get_docker_util();
    let result = docker_util.stop_container("test_container");

    assert!(result.is_ok())
}
