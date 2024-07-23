use crate::error::DockerError;
use crate::DockerUtil;
use common_container::prelude::ContainerConfig;
use std::process::Command;

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
        &self,
        container_config: &ContainerConfig,
    ) -> Result<(String, u16), DockerError> {
        self.get_or_start_container(container_config)
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
    ////// ```
    pub(crate) fn get_or_start_container(
        &self,
        container_config: &ContainerConfig,
    ) -> Result<(String, u16), DockerError> {
        // Unpack values from container config
        let name = container_config.name();
        let image = &container_config.container_image();
        let connection_port = container_config.connection_port();
        let additional_ports = container_config.additional_ports();
        let platform = container_config.platform();
        let additional_env_vars = container_config.additional_env_vars();
        let reuse_container = container_config.reuse_container();

        let container_id = &format!("{}-{}", name, connection_port);

        println!("Container ID: {}", container_id);

        self.dbg_print("Check if container already exists.");
        let exists = self
            .check_if_container_exists(container_id)
            .expect("Failed to check if container exists");

        if exists {
            self.dbg_print("Container already exists.");
            if reuse_container {
                self.dbg_print("Re-using running container.");
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
        self.dbg_print("Pull container image.");
        match self.pull_container_image(container_id, image, platform) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        self.dbg_print("Start new container.");
        match self.start_container(
            container_id,
            connection_port,
            additional_ports,
            platform,
            additional_env_vars,
            image,
        ) {
            Ok((container_id, port)) => Ok((container_id, port)),
            Err(e) => Err(e),
        }
    }

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
    pub(crate) fn start_container(
        &self,
        container_id: &str,
        connection_port: u16,
        additional_ports: Option<&[u16]>,
        platform: Option<&str>,
        additional_env_vars: Option<&[&str]>,
        image: &str,
    ) -> Result<(String, u16), DockerError> {
        // Example: docker run --rm --detach --publish 80:80 --name test-80 nginx:latest
        self.dbg_print(&format!(
            "[start_container]: Starting new container: {}.",
            container_id
        ));

        // construct initial command
        let mut cmd = Command::new("docker");

        cmd.arg("run").arg("--rm").arg("--detach");

        if platform.is_some() {
            let p = platform.expect("Failed to unwrap Docker platform string");
            cmd.arg("--platform").arg(p);
        }

        // Format main connection port for docker
        let port_publish = format!("{}:{}", connection_port, connection_port);
        cmd.arg("--publish").arg(port_publish);

        // Publish additional ports for the container, if applicable
        if additional_ports.is_some() {
            for port in additional_ports.expect("Failed to unwrap additional Docker ports") {
                if *port == 0 {
                    return Err(DockerError::from(format!(
                        "Error starting container {}: Port cannot be 0.",
                        container_id,
                    )));
                }

                // Example: --publish 80:80
                // Format port for docker
                let port_publish = format!("{}:{}", port, port);
                // Add argument
                cmd.arg("--publish").arg(port_publish);
            }
        }

        // Format the container name
        let container_name = container_id.to_string();

        // Add container name
        cmd.arg("--name");
        cmd.arg(container_name);

        // Add env variables, if available
        if additional_env_vars.is_some() {
            // Add additional env variables
            let add_args = additional_env_vars.unwrap();

            cmd.arg("-e");
            cmd.args(add_args);
        }

        // Add container image to start
        cmd.arg(image);

        self.dbg_print(&format!("[start_container]: Run Docker command: {:?}", cmd));

        // There are multiple ways to spawn a child process and execute an arbitrary command on the machine:
        //
        // spawn — runs the program and returns a value with details
        // output — runs the program and returns the output
        // status — runs the program and returns the exit code |  io::Result<ExitStatus>
        // https://stackoverflow.com/questions/21011330/how-do-i-invoke-a-system-command-and-capture-its-output

        // Run the command & return error in case of failure
        match cmd.output() {
            Ok(out) => {
                self.dbg_print(&format!(
                    "[start_container]: \n
                    success: {} \n
                    Output: {}",
                    out.status.success(),
                    String::from_utf8_lossy(out.stdout.as_slice()),
                ));
                Ok((container_id.to_string(), connection_port))
            }
            Err(e) => Err(DockerError::from(format!(
                "Error starting container {}: {}",
                container_id, e
            ))),
        }
    }
}
