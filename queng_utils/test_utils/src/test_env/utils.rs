use crate::docker_util::DockerUtil;
use crate::prelude::DockerError;

pub(crate) fn get_docker_util() -> Result<DockerUtil, DockerError> {
    return match DockerUtil::new() {
        Ok(docker_util) => Ok(docker_util),
        Err(e) => Err(e),
    };
}
