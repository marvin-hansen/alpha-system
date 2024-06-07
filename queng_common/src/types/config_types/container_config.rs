use std::fmt::Display;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ContainerConfig<'l> {
    name: &'l str,
    image: &'l str,
    tag: &'l str,
    url: &'l str,
    connection_port: u16,
    additional_ports: Option<&'l [u16]>,
    platform: Option<&'l str>,
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
    /// * `tag` - The tag of the image.
    /// * `url` - The default URL of the container. Usually 0.0.0.0
    /// * `connection_port` - The port number for the main connection i.e. 80 for a webserver.
    /// * `additional_ports` - An optional array of additional ports to publish.
    /// * `platform` - An optional platform string in case the container image is not multi-arch.
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
    /// use common::prelude::ContainerConfig;
    ///
    /// let container_config = ContainerConfig::new(
    ///     "my_container","nginx",":latest", "0.0.0.0" ,80, None, None, false, false
    /// );
    /// ```
    pub fn new(
        name: &'l str,
        image: &'l str,
        tag: &'l str,
        url: &'l str,
        connection_port: u16,
        additional_ports: Option<&'l [u16]>,
        platform: Option<&'l str>,
        reuse_container: bool,
        reset_configuration: bool,
    ) -> Self {
        Self {
            name,
            image,
            tag,
            url,
            connection_port,
            additional_ports,
            platform,
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
    pub fn connection_port(&self) -> u16 {
        self.connection_port
    }
    pub fn additional_ports(&self) -> Option<&'l [u16]> {
        self.additional_ports
    }
    pub fn platform(&self) -> Option<&'l str> {
        self.platform
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
            "name: {}, image: {}:{}, url: {} connection_port: {}, additional_ports: {:?}, reuse_container: {}, reset_configuration: {}",
            self.name,
            self.image,
            self.tag,
            self.url,
            self.connection_port,
            self.additional_ports,
            self.reuse_container,
            self.reset_configuration,
        )
    }
}
