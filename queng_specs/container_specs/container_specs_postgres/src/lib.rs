use common_container::prelude::{ContainerConfig, WaitStrategy};

pub fn postgres_db_container_config() -> ContainerConfig<'static> {
    // Ensure name matches exactly the generated name by the DockerUtil
    ContainerConfig::new(
        "postgres",
        "postgres",
        //  When you update the Dockertag,
        // also update the postgres.sh script in scripts/ folder
        "17-alpine3.20",
        "0.0.0.0",
        5432,
        None,
        Some(&["POSTGRES_PASSWORD=postgres"]),
        None,
        true, // Keep the container running for re-use
        true, // Keep the same container config across all env. setups.
        WaitStrategy::WaitUntilConsoleOutputContains(
            "PostgreSQL init process complete; ready for start up.".to_string(),
            30,
        ),
    )
}
