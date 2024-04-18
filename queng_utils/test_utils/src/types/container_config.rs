use std::fmt::Display;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ContainerConfig<'l> {
    name: &'l str,
    image: &'l str,
    tag: &'l str,
    port: u16,
    reuse_container: bool,
}

impl<'l> ContainerConfig<'l> {
    /// Create a new instance of the `ContainerConfig` struct with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the container.
    /// * `image` - The image to use for the container.
    /// * `tag` - The tag of the image.
    /// * `port` - The port number for the container.
    /// * `reuse_container` - A boolean flag indicating whether to reuse an existing container if found.
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
    /// let container_config = ContainerConfig::new("my_container", "nginx",":latest" ,80, false);
    /// ```
    pub fn new(
        name: &'l str,
        image: &'l str,
        tag: &'l str,
        port: u16,
        reuse_container: bool,
    ) -> Self {
        Self {
            name,
            image,
            tag,
            port,
            reuse_container,
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
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn reuse_container(&self) -> bool {
        self.reuse_container
    }
}

impl Display for ContainerConfig<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, image: {}:{}, port: {}, reuse_container: {}",
            self.name, self.image, self.tag, self.port, self.reuse_container
        )
    }
}
