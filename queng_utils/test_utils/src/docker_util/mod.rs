use std::fmt::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DockerUtil {}

impl DockerUtil {
    pub fn new() -> Self {
        Self {}
    }
}

impl DockerUtil {
    pub fn get_running_container() -> Option<(u16, String)> {
        Some((0, "".to_string()))
    }

    pub fn start_container(
        name: &str,
        image: &str,
        port: u16,
        reuse_server: bool,
    ) -> Result<(u16, String), Error> {
        Ok((0, "".to_string()))
    }

    pub fn stop_container() -> Result<(), Error> {
        Ok(())
    }
}
