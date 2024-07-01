use common::prelude::ContainerConfig;

/// Constructs the configuration for an API proxy container.
///
/// This function provides a `ContainerConfig` with predefined settings necessary for
/// setting up an API proxy container.
/// # Returns
/// A `ContainerConfig` instance containing all the necessary configuration for the API proxy container.
pub fn api_proxy_container_config() -> ContainerConfig<'static> {
    // Private container image
    // https://console.cloud.google.com/artifacts/docker/future-309012/asia-northeast1/image-repo/kaiko_proxy?project=future-309012
    ContainerConfig::new(
        "apiproxy",
        "asia-northeast1-docker.pkg.dev/future-309012/image-repo/kaiko_proxy",
        "ef0f5da",
        "0.0.0.0",
        7777,
        None,
        Some("linux/amd64"),
        true,  // Keep the container running for re-use
        false, // Keep the same container config across all env. setups.
        10,    // Wait a few seconds until the container finished starting up.
    )
}
