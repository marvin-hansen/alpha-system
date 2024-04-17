use crate::prelude::DockerError;
use std::process::Command;

// There are multiple ways to spawn a child process and execute an arbitrary command on the machine:
//
// spawn — runs the program and returns a value with details
// output — runs the program and returns the output
// status — runs the program and returns the exit code |  io::Result<ExitStatus>
// https://stackoverflow.com/questions/21011330/how-do-i-invoke-a-system-command-and-capture-its-output

const DBG: bool = true;

fn dbg_print(s: &str) {
    if DBG {
        println!("[DockerUtil]: {}", s);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DockerUtil {}

impl DockerUtil {
    pub fn new() -> Result<Self, DockerError> {
        return match Command::new("docker").arg("-v").status() {
            Ok(_) => Ok(Self {}),
            Err(e) => Err(DockerError::from(format!(
                "Error connecting to Docker: {}",
                e
            ))),
        };
    }
}

impl DockerUtil {
    /// Start a container
    pub fn get_or_start_container(
        &mut self,
        name: &str,
        image: &str,
        port: u16,
        reuse_server: bool,
    ) -> Result<(String, u16), DockerError> {
        //
        dbg_print(" Check if container already exists.");
        let exists = self
            .check_if_container_exists(name)
            .expect("Failed to check if container exists");

        if exists {
            dbg_print(" Container already exists.");
            if reuse_server {
                dbg_print("Re-using running running container.");
                return match self.get_running_container(name) {
                    Ok((container_name, port)) => Ok((container_name, port)),
                    Err(e) => return Err(e),
                };
            }

            dbg_print("Stopping running container b/c no re-use wanted.");
            self.stop_container(name).expect("Failed to stop container");
        }

        dbg_print("Container doesn't exist.");
        dbg_print("Start new container.");
        return match self.start_container(name, port, image) {
            Ok((container_id, port)) => Ok((container_id, port)),
            Err(e) => Err(e),
        };
    }

    /// Stop a container
    pub fn stop_container(&mut self, container_id: &str) -> Result<(), DockerError> {
        dbg_print(" Check if container already exists.");
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
            dbg_print(" Container already exists. Stopping it.");
            // Example: docker kill test-80
            return match Command::new("docker kill").arg(container_id).status() {
                Ok(_) => Ok(()),
                Err(e) => Err(DockerError::from(format!(
                    "Error stopping container {}: {}",
                    container_id,
                    e.to_string()
                ))),
            };
        }

        Ok(())
    }
}

impl DockerUtil {
    /// Start a container
    fn start_container(
        &self,
        container_id: &str,
        port: u16,
        image: &str,
    ) -> Result<(String, u16), DockerError> {
        // Example: docker run --rm --detach --publish 80:80 --name test-80 nginx:latest
        return match Command::new("docker run")
            .arg("--rm")
            .arg("--detach")
            .arg(format!("--publish {}:{}", port, port))
            .arg(format!("--name {}", container_id))
            .arg(image)
            .output()
        {
            Ok(out) => {
                dbg_print(&format!("{}", String::from_utf8_lossy(&out.stdout)));
                Ok((container_id.to_string(), port))
            }
            Err(e) => Err(DockerError::from(format!(
                "Error starting container {}: {}",
                container_id,
                e.to_string()
            ))),
        };
    }

    /// Either returns the name and port of a container if its running, otherwise an error.
    fn get_running_container(&self, container_id: &str) -> Result<(String, u16), DockerError> {
        let container = match Command::new("docker ps")
            .arg(format!("--filter=name={}", container_id))
            .arg("--format={{.Names}}")
            .output()
        {
            Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
            Err(e) => {
                return Err(DockerError::from(format!(
                    "Error getting container {}: {}",
                    container_id,
                    e.to_string()
                )));
            }
        };

        if container.is_empty() {
            return Err(DockerError::from(format!(
                "Error no container found for ID: {}",
                container_id,
            )));
        }

        let parts = container.split("-").collect::<Vec<&str>>();

        let container_name = parts
            .first()
            .expect("Failed to get container name")
            .to_string();

        let port = parts
            .last()
            .expect("Failed to get container port")
            .parse::<u16>()
            .expect("Failed to convert container port from string into u16");

        return Ok((container_name, port));
    }

    /// Check if a container is running
    fn check_if_container_exists(&mut self, container_id: &str) -> Result<bool, DockerError> {
        return match self.get_running_container(container_id) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        };
    }
}
