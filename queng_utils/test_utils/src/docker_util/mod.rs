use crate::prelude::DockerError;
use docker_engine_api::client::{Client, ClientTrait};
use docker_engine_api::container_create::CreateContainerFrom;
use docker_engine_api::container_inspect::InspectedContainer;
use docker_engine_api::containers_service::ContainersServiceTrait;
use std::collections::HashMap;

const DEFAULT_PLATFORM: &str = "linux";
const DBG: bool = true;

fn dbg_print(s: &str) {
    if DBG {
        println!("[DockerUtil]: {}", s);
    }
}

pub struct DockerUtil {
    client: Client,
}

impl DockerUtil {
    pub fn new() -> Result<Self, DockerError> {
        let client = docker_engine_api::new("/var/run/docker.sock".to_string());
        match client.ping() {
            Ok(_) => {
                dbg_print("Connected to Docker");
            }
            Err(e) => {
                println!("Failed to connect to Docker: {}", e);
                return Err(DockerError::from(e.to_string()));
            }
        };

        Ok(Self { client })
    }
}

impl DockerUtil {
    /// Check if a container is running
    pub fn check_if_container_exists(&mut self, container_id: &str) -> Result<bool, DockerError> {
        // Somehow the client API has no way build in to check if a container is exists.
        // So we have to improvise by calling the stats endpoint b/c no container, no stats...
        return match self
            .client
            .containers
            .get_stats_container(container_id, false, true)
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        };
    }

    /// Check if the container is running
    pub fn check_if_container_is_running(
        &mut self,
        container_id: &str,
    ) -> Result<bool, DockerError> {
        // Check if container exists.
        let exists = self
            .check_if_container_exists(container_id)
            .expect("Failed to check if container exists");

        // If container doesn't exists, return an error
        if !exists {
            return Err(DockerError::from(format!(
                "Container doesn't exists: {}",
                container_id
            )));
        }

        // Get status report for container
        let report = self
            .get_container_report(container_id)
            .expect("Failed to get container status report");

        if report.state.running {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Implement this
    pub fn get_running_container(&self) -> Result<(u16, String), DockerError> {
        return Err(DockerError::from("NOT IMPLEMENTED"));
    }

    /// Start a container
    pub fn start_container(
        &mut self,
        name: &str,
        image: &str,
        cmd: Option<Vec<String>>,
        port: u16,
        reuse_server: bool,
    ) -> Result<(u16, String), DockerError> {
        //
        // Check if container already exists.
        let exists = self
            .check_if_container_exists(name)
            .expect("Failed to check if container exists");

        if exists {
            // Check if container is already running
            let running = self
                .check_if_container_is_running(name)
                .expect("Failed to check if container is running");

            // if the container is already running
            if running {
                // and if we want to re-use the running container
                if reuse_server {
                    // Return the active container name and port

                    //
                    // implement get running
                    let (port, container_name) = match self.get_running_container() {
                        Ok((port, container_name)) => (port, container_name),
                        Err(e) => return Err(e),
                    };

                    return Ok((port, container_name));
                }

                // Because we don't re-use the server,
                // we need to stop the container first
                self.stop_container(name).expect("Failed to stop container");
            }
        }

        // Container doesn't exist, so let's create one.
        // Define options
        let mut options = CreateContainerFrom::default();
        options.image = Some(image.to_string());
        options.cmd = cmd;

        // Define exposed ports
        let mut exposed_ports = HashMap::new();
        exposed_ports.insert(port.to_string(), ());
        // Expose the metric port if it doesn't conflict with the service port.
        if port != 8080 {
            exposed_ports.insert("8080/tcp".to_string(), ());
        }
        // Set the exposed ports
        options.exposed_ports = Some(exposed_ports);

        // Call to create a container for the provided image name
        let container_id =
            match self
                .client
                .containers
                .create_container(name, DEFAULT_PLATFORM, &options)
            {
                Ok(re) => re.id,
                Err(e) => return Err(DockerError::from(e.to_string())),
            };

        // Start the container
        match self.client.containers.start_container(&container_id) {
            Ok(res) => res,
            Err(e) => return Err(DockerError::from(e.to_string())),
        };

        Ok((port, container_id))
    }

    /// Stop a container
    pub fn stop_container(&mut self, container_id: &str) -> Result<(), DockerError> {
        // Check if container already exists.
        let exists = self
            .check_if_container_exists(container_id)
            .expect("Failed to check if container exists");

        if !exists {
            return Err(DockerError::from(format!(
                "Container doesn't exists: {}",
                container_id
            )));
        }

        if exists {
            // Check if container is already running
            let running = self
                .check_if_container_is_running(container_id)
                .expect("Failed to check if container is running");

            // if the container is running, if so, stop it
            if running {
                match self.client.containers.stop_container(container_id, 30) {
                    Ok(_) => (),
                    Err(e) => return Err(DockerError::from(e.to_string())),
                }
            }
        }

        Ok(())
    }
}

impl DockerUtil {
    fn get_container_report(
        &mut self,
        container_id: &str,
    ) -> Result<InspectedContainer, DockerError> {
        return match self
            .client
            .containers
            .inspect_container(container_id, false)
        {
            Ok(report) => Ok(report),
            Err(e) => Err(DockerError::from(e.to_string())),
        };
    }
}
