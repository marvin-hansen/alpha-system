use common_container::{ContainerConfig, WaitStrategy};

pub fn iggy_container_config() -> ContainerConfig<'static> {
    ContainerConfig::new(
        "iggy",
        "iggyrs/iggy",
        //  When you update the Dockertag,
        // also update the iggy.sh script in scripts/ folder
        "0.4.84",
        "0.0.0.0",
        8090,
        Some(&[8080, 8090]),
        None,
        None,
        true, // Keep the container running for re-use
        true, // Keep the same container config across all env. setups.
        WaitStrategy::WaitUntilConsoleOutputContains(
            "Iggy TCP server has started on: 0.0.0.0:8090".to_string(),
            10,
        ),
    )
}
