use common_container::{ContainerConfig, WaitStrategy};

pub fn iggy_container_config() -> ContainerConfig<'static> {
    ContainerConfig::new(
        "iggy",
        "iggyrs/iggy",
        //  When you update the Dockertag,
        // also update the iggy.sh script in scripts/ folder
        "0.4.84",
        "0.0.0.0",
        3000,
        None,
        None,
        None,
        true, // Keep the container running for re-use
        true, // Keep the same container config across all env. setups.
        WaitStrategy::WaitUntilConsoleOutputContains(
            "Started HTTP API on: 0.0.0.0:3000".to_string(),
            10,
        ),
    )
}
