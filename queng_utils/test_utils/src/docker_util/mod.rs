use crate::prelude::DockerError;

use docker_engine_api::client::{Client, ClientTrait};

pub struct DockerUtil {
    client: Client,
}

impl DockerUtil {
    pub fn new() -> Self {
        let mut client = docker_engine_api::new("/var/run/docker.sock".to_string());
        match client.ping() {
            Ok(_) => {
                println!("Pong!")
            }
            Err(e) => panic!("Error: {}", e),
        };

        Self { client }
    }
}

impl DockerUtil {
    /// Check if a container is running
    ///
    pub fn is_container_running(&self, name: &str) -> Result<bool, DockerError> {
        return Err(DockerError::from("NOT IMPLEMENTED"));
    }

    pub fn get_running_container(&self) -> Result<(u16, String), DockerError> {
        return Err(DockerError::from("NOT IMPLEMENTED"));
    }

    /// Start a container
    pub fn start_container(
        &self,
        name: &str,
        image: &str,
        port: u16,
        reuse_server: bool,
    ) -> Result<(u16, String), DockerError> {
        //
        // Check if container is already running
        let running = self
            .is_container_running(name)
            .expect("Failed to check if container is running");

        if reuse_server && running {
            let (port, container_name) = match self.get_running_container() {
                Ok((port, container_name)) => (port, container_name),
                Err(e) => return Err(e),
            };

            return Ok((port, container_name));
        }

        // Because we don't re-use the server, we need to stop the container first
        if running {
            self.stop_container(name).expect("Failed to stop container");
        }

        // Start the container

        return Err(DockerError::from("NOT IMPLEMENTED"));
    }

    /// Stop a container
    pub fn stop_container(&self, name: &str) -> Result<(), DockerError> {
        return Err(DockerError::from("NOT IMPLEMENTED"));
    }
}

impl DockerUtil {
    /// Find a free port
    fn find_free_port() -> u16 {
        0
    }
}
