use common::prelude::ContainerConfig;

/// Constructs the configuration for an API proxy container.
///
/// This function provides a `ContainerConfig` with predefined settings necessary for
/// setting up an API proxy container.
/// # Returns
/// A `ContainerConfig` instance containing all the necessary configuration for the API proxy container.
pub fn api_proxy_container_config() -> ContainerConfig<'static> {
    ContainerConfig::new(
        "apiproxy",
        "index.docker.io/hansenmarvin/api_proxy",
        "d9f20cd",
        "0.0.0.0",
        7777,
        None,
        Some("linux/amd64"),
        None,
        true, // Keep the container running for re-use
        true, // Keep the same container config across all env. setups.
        12,   // Wait a few seconds until the container finished starting up.
    )
}
