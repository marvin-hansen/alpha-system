use common::prelude::ContainerConfig;

pub fn api_proxy_container_config() -> ContainerConfig<'static> {
    // Private container image
    // https://console.cloud.google.com/artifacts/docker/future-309012/asia-northeast1/image-repo/kaiko_proxy?project=future-309012
    ContainerConfig::new(
        "apiproxy",
        "asia-northeast1-docker.pkg.dev/future-309012/image-repo/kaiko_proxy",
        "a0a9652",
        "127.0.0.1",
        7777,
        None,
        Some("linux/amd64"),
        true,  // Keep the container running for re-use
        false, // Keep the same container config across all env. setups.
    )
}
