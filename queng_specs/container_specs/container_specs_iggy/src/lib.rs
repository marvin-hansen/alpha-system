use docker_utils::{ContainerConfig, WaitStrategy};

pub fn iggy_container_config() -> ContainerConfig<'static> {
    ContainerConfig::new(
        "ci_iggy",
        "ghcr.io/marvin-hansen/ci_iggy/ci_iggy",
        //  When you update the Dockertag,
        // also update the iggy.sh script in scripts/ folder
        "latest",
        "0.0.0.0",
        3000,
        Some(&[8090]),
        None,
        None,
        true, // Keep the container running for re-use
        true, // Keep the same container config across all env. setups.
        WaitStrategy::WaitForHttpHealthCheck("http://0.0.0.0:3000/ping".to_string(), 15),
    )
}
