use crate::prelude::{ContainerConfig, DockerError};
use std::process::Command;

// There are multiple ways to spawn a child process and execute an arbitrary command on the machine:
//
// spawn — runs the program and returns a value with details
// output — runs the program and returns the output
// status — runs the program and returns the exit code |  io::Result<ExitStatus>
// https://stackoverflow.com/questions/21011330/how-do-i-invoke-a-system-command-and-capture-its-output

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DockerUtil {
    dbg: bool,
}

impl DockerUtil {
    /// Create a new instance of the `DockerUtil` struct.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the `DockerUtil` struct with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use test_utils::prelude::DockerUtil;
    ///
    /// // Requires running Docker. Start Docker and uncomment.
    /// //let docker_util = DockerUtil::new();
    /// ```
    ///
    pub fn new() -> Result<Self, DockerError> {
        Self::build(false)
    }

    /// Create a new instance of the `DockerUtil` struct with debug mode enabled.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a new instance of the `DockerUtil` struct with debug mode enabled, or a `DockerError` if an error occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// use test_utils::prelude::DockerUtil;
    ///
    /// // Requires running Docker. Start Docker and uncomment.
    /// //let docker_util = DockerUtil::with_debug().expect("Failed to create DockerUtil with debug mode");
    /// ```
    pub fn with_debug() -> Result<Self, DockerError> {
        Self::build(true)
    }

    /// Build a new instance of the `DockerUtil` struct with the given debug flag.
    ///
    /// # Arguments
    ///
    /// * `dbg` - A boolean flag indicating whether to enable debug mode.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a new instance of the `DockerUtil` struct if successful, or a `DockerError` if an error occurred.
    ///
    fn build(dbg: bool) -> Result<Self, DockerError> {
        return match Command::new("docker").arg("-v").spawn() {
            Ok(_) => Ok(Self { dbg }),
            Err(e) => Err(DockerError::from(format!(
                "Error connecting to Docker: {}",
                e
            ))),
        };
    }
}

impl Default for DockerUtil {
    fn default() -> Self {
        Self::new().expect("Failed to create DockerUtil")
    }
}

impl DockerUtil {
    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[DockerUtil]: {}", s);
        }
    }
}

impl DockerUtil {
    /// Gets an existing container or starts a new one with the specified configuration
    ///
    /// # Arguments
    ///
    /// * `container_config` - The configuration of the container.
    ///
    /// # Returns
    ///
    /// Returns a tuple containing the container name and port if successful,
    /// or a `DockerError` if an error occurs.
    ///
    pub fn get_or_start_container_config(
        &mut self,
        container_config: &ContainerConfig,
    ) -> Result<(String, u16), DockerError> {
        // Unpack values from container config
        let name = container_config.name();
        let image = &container_config.container_image();
        let port = container_config.port();
        let reuse_container = container_config.reuse_container();

        // Call get_or_start_container with unpacked values
        self.get_or_start_container(name, image, port, reuse_container)
    }

    /// Gets an existing container or starts a new one with the specified name, image, port, and reuse status.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the container.
    /// * `image` - The image to use for the container.
    /// * `port` - The port number for the container.
    /// * `reuse_container` - A boolean flag indicating whether to reuse an existing container if found.
    ///
    /// # Returns
    ///
    /// Returns a tuple containing the container name and port if successful, or a `DockerError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// use test_utils::prelude::DockerUtil;
    ///
    /// let name = "nginx";
    /// let image = "nginx:latest";
    /// let port = 80;
    /// let reuse_container = false;
    ///
    /// // Requires running Docker. Start Docker and uncomment.
    /// //let mut docker_util = DockerUtil::new().expect("Failed to create DockerUtil");
    /// //let result = docker_util.get_or_start_container(name, image, port, reuse_container);
    /// ```
    pub fn get_or_start_container(
        &mut self,
        name: &str,
        image: &str,
        port: u16,
        reuse_container: bool,
    ) -> Result<(String, u16), DockerError> {
        let container_id = &format!("{}-{}", name, port);

        println!("Container ID: {}", container_id);

        self.dbg_print("Check if container already exists.");
        let exists = self
            .check_if_container_exists(container_id)
            .expect("Failed to check if container exists");

        if exists {
            self.dbg_print("Container already exists.");
            if reuse_container {
                self.dbg_print("Re-using running running container.");
                return match self.get_running_container(container_id) {
                    Ok((container_name, port)) => Ok((container_name, port)),
                    Err(e) => return Err(e),
                };
            }

            self.dbg_print("Stopping running container b/c no re-use wanted.");
            self.stop_container(container_id)
                .expect("Failed to stop container");
        }

        self.dbg_print("Container doesn't exist.");
        self.dbg_print("Start new container.");
        return match self.start_container(container_id, port, image) {
            Ok((container_id, port)) => Ok((container_id, port)),
            Err(e) => Err(e),
        };
    }

    /// Stop a container
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID of the container to stop.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the container was successfully stopped, or `Err(DockerError)` if an error occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// use test_utils::prelude::DockerUtil;
    ///
    /// // Requires running Docker. Start Docker and uncomment.
    /// // let mut docker_util = DockerUtil::new().expect("Failed to create DockerUtil");
    /// // let container_id = "my_container";
    /// // docker_util.stop_container(container_id).expect("Failed to stop container");
    /// ```
    pub fn stop_container(&mut self, container_id: &str) -> Result<(), DockerError> {
        self.dbg_print("[stop_container]: Check if container exists.");
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
            self.dbg_print("[stop_container]: Container exists. Stopping it.");
            // Example: docker kill test-80
            return match Command::new("docker")
                .arg("kill")
                .arg(container_id)
                .status()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(DockerError::from(format!(
                    "[stop_container]: Error stopping container {}: {}",
                    container_id,
                    e.to_string()
                ))),
            };
        }

        Ok(())
    }
}

impl DockerUtil {
    /// Start a stopped container by its ID.
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID of the container to start.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the container was successfully started, or `Err(DockerError)` if an error occurred.
    ///
    pub fn start_container(
        &self,
        container_id: &str,
        port: u16,
        image: &str,
    ) -> Result<(String, u16), DockerError> {
        // Example: docker run --rm --detach --publish 80:80 --name test-80 nginx:latest
        self.dbg_print(&format!(
            "[start_container]: Starting new container: {}.",
            container_id
        ));

        let port_publish = format!("{}:{}", port, port);
        let container_name = format!("{}", container_id);

        return match Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("--detach")
            // .arg("--network=host")
            .arg("--publish")
            .arg(port_publish)
            .arg("--name")
            .arg(container_name)
            .arg(image)
            .status()
        {
            Ok(_) => {
                self.dbg_print(&format!(
                    "[start_container]: {}",
                    container_id //String::from_utf8_lossy(&out.stdout)
                ));
                Ok((container_id.to_string(), port))
            }
            Err(e) => Err(DockerError::from(format!(
                "Error starting container {}: {}",
                container_id,
                e.to_string()
            ))),
        };
    }

    /// Get information about a running container by its ID.
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID of the container to retrieve information about.
    ///
    /// # Returns
    ///
    /// Either returns the name and port of a container if its running, otherwise an DockerError.
    ///
    fn get_running_container(&self, container_id: &str) -> Result<(String, u16), DockerError> {
        let container = match Command::new("docker")
            .arg("ps")
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
        let port = parts
            .last()
            .expect("Failed to get container port")
            .trim()
            .parse::<u16>()
            .expect("Failed to convert container port from string into u16");

        return Ok((container.trim().to_string(), port));
    }

    /// Check if a container exists by its ID.
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID of the container to check.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the container exists, `Ok(false)` if the container does not exist, or `Err(DockerError)` if an error occurred.
    ///
    pub fn check_if_container_exists(&mut self, container_id: &str) -> Result<bool, DockerError> {
        return match self.get_running_container(container_id) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        };
    }
}
