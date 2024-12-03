use common_container::{ContainerConfig, WaitStrategy};

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
        Some(&["RUST_LOG=debug"]),
        None,
        true, // Keep the container running for re-use
        true, // Keep the same container config across all env. setups.
        WaitStrategy::WaitUntilConsoleOutputContains(
            "Started HTTP API on: 0.0.0.0:3000".to_string(),
            20,
        ),
    )
}
