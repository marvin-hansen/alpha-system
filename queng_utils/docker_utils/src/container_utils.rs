use crate::error::DockerError;
use crate::DockerUtil;
use std::process::Command;

impl DockerUtil {
    /// Pulls a container image from registry
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID of the container to start.
    /// * `image` - The container image with tag
    /// * `platform` - Optional platform tag i.e. linux/amd64
    ///
    pub fn pull_container_image(
        &self,
        container_id: &str,
        image: &str,
        platform: Option<&str>,
    ) -> Result<(), DockerError> {
        // Example docker pull --platform linux/amd64  asia-northeast1-docker.pkg.dev/future-309012/image-repo/api_proxy:b422ae3
        self.dbg_print(&format!(
            "[pull_container_image]: Pull container image for: {}.",
            container_id
        ));

        // construct initial command
        let mut cmd = Command::new("docker");
        cmd.arg("pull");

        if platform.is_some() {
            let p = platform.expect("Failed to unwrap Docker platform string");
            cmd.arg("--platform").arg(p);
        }

        // Add the image
        cmd.arg(image);

        self.dbg_print(&format!("[pull_container_image]: Pull command: {:?}.", cmd));

        // Run the command & return error in case of failure
        match cmd.output() {
            Ok(out) => {
                self.dbg_print(&format!(
                    "[pull_container_image]: success. Image Pulled {}",
                    image
                ));
                self.dbg_print(&format!(
                    "[pull_container_image]: success. Status: {} Message: {:?}",
                    out.status, out.stderr
                ));
                Ok(())
            }
            Err(e) => {
                eprintln!();
                eprintln!("Error pulling container image {}: {}", container_id, e);
                eprintln!();
                panic!("")
            }
        }
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
    pub fn get_running_container(&self, container_id: &str) -> Result<(String, u16), DockerError> {
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
                    container_id, e
                )));
            }
        };

        if container.is_empty() {
            return Err(DockerError::from(format!(
                "Error no container found for ID: {}",
                container_id,
            )));
        }

        let parts = container.split('-').collect::<Vec<&str>>();
        let port = parts
            .last()
            .expect("Failed to get container port")
            .trim()
            .parse::<u16>()
            .expect("Failed to convert container port from string into u16");

        return Ok((container.trim().to_string(), port));
    }

    pub fn get_running_container_image_tag(
        &self,
        container_id: &str,
    ) -> Result<String, DockerError> {
        match self.check_if_container_exists(container_id) {
            Ok(_) => {}
            Err(_) => {
                return Err(DockerError::from(format!(
                    "[get_container_image_tag]: Error no container found for ID: {}",
                    container_id,
                )));
            }
        }

        let container_image = match Command::new("docker")
            .arg("ps")
            .arg(format!("--filter=name={}", container_id))
            .arg("--format={{.Image}}")
            .output()
        {
            Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
            Err(e) => {
                return Err(DockerError::from(format!(
                    "[get_container_image_tag]: Error getting container image for {}: {}",
                    container_id, e
                )));
            }
        };

        if container_image.is_empty() {
            return Err(DockerError::from(format!(
                "[get_container_image_tag]: Error no image found for container ID: {}",
                container_id,
            )));
        }

        let parts = container_image.split(':').collect::<Vec<&str>>();
        let tag = parts
            .last()
            .expect("[get_container_image_tag]: Failed to get container tag")
            .trim()
            .to_owned();

        Ok(tag)
    }

    pub fn check_if_running_container_uses_target_tag(
        &self,
        container_id: &str,
        target_tag: &str,
    ) -> Result<bool, DockerError> {
        return match self.get_running_container_image_tag(container_id) {
            Ok(container_tag) => {
                Ok(container_tag.eq_ignore_ascii_case(target_tag))
            }
            Err(e) => {
                Err(DockerError::from(format!(
                    "[check_if_container_uses_target_tag]: Error getting container_tag for container ID: {} {}",
                    container_id, e.to_string()
                )))
            }
        };
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
    pub fn check_if_container_exists(&self, container_id: &str) -> Result<bool, DockerError> {
        match self.get_running_container(container_id) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn prune_containers(&mut self) -> Result<(), DockerError> {
        return match Command::new("docker")
            .arg("system")
            .arg("prune")
            .arg("--all")
            .arg("--force")
            .spawn()
        {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(DockerError::from(format!(
                    "Error pruning containers: {}",
                    e
                )));
            }
        };
    }
}
