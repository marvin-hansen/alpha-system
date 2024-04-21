use std::fmt::Display;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ContainerConfig<'l> {
    name: &'l str,
    image: &'l str,
    tag: &'l str,
    url: &'l str,
    port: u16,
    reuse_container: bool,
    reset_configuration: bool,
}

impl<'l> ContainerConfig<'l> {
    /// Create a new instance of the `ContainerConfig` struct with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the container.
    /// * `image` - The image to use for the container.
    /// * `url` - The default URL of the container. Usually 0.0.0.0
    /// * `tag` - The tag of the image.
    /// * `port` - The port number for the container.
    /// * `reuse_container` - A boolean flag indicating whether to reuse an existing container if found.
    /// * `reset_configuration` -  A boolean flag indication whether to reset the configuration upon
    ///    every environment setup. If set to false, the same configuration will be used across all
    ///    environment setups.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the `ContainerConfig` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use test_utils::prelude::ContainerConfig;
    ///
    /// let container_config = ContainerConfig::new("my_container", "nginx",":latest", "0.0.0.0" ,80, false, false);
    /// ```
    pub fn new(
        name: &'l str,
        image: &'l str,
        tag: &'l str,
        url: &'l str,
        port: u16,
        reuse_container: bool,
        reset_configuration: bool,
    ) -> Self {
        Self {
            name,
            image,
            tag,
            url,
            port,
            reuse_container,
            reset_configuration,
        }
    }
}

impl<'l> ContainerConfig<'l> {
    pub fn name(&self) -> &'l str {
        self.name
    }
    pub fn container_image(&self) -> String {
        format!("{}:{}", self.image, self.tag)
    }
    pub fn url(&self) -> &'l str {
        self.url
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn reuse_container(&self) -> bool {
        self.reuse_container
    }
    pub fn reset_configuration(&self) -> bool {
        self.reset_configuration
    }
}

impl Display for ContainerConfig<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, image: {}:{}, url: {} port: {}, reuse_container: {}, reset_configuration: {}",
            self.name,
            self.image,
            self.tag,
            self.url,
            self.port,
            self.reuse_container,
            self.reset_configuration,
        )
    }
}
