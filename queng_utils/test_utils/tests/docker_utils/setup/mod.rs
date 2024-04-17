use test_utils::prelude::DockerUtil;

pub fn get_docker_util() -> DockerUtil {
    DockerUtil::new().expect("Failed to create DockerUtil")
}

#[test]
fn test_new() {
    let result = DockerUtil::new();

    if result.is_err() {
        println!("{}", result.as_ref().unwrap_err());
    }

    assert!(result.is_ok());
}

// #[test]
// fn test_start_container_no_reuse() {
//     let mut docker_util = get_docker_util();
//     let result = docker_util.start_container(
//         "test_container",
//         "alpine:latest",
//         Some(vec!["/bin/true".to_string()]),
//         7070,
//         false,
//     );
//
//     assert!(result.is_ok())
// }
