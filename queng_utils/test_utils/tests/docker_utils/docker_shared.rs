use test_utils::prelude::DockerUtil;

pub fn get_docker_util() -> DockerUtil {
    DockerUtil::new().expect("Failed to create DockerUtil")
}
