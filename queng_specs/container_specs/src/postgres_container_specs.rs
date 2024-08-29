use common_container::prelude::{ContainerConfig, WaitStrategy};

pub fn postgres_db_container_config() -> ContainerConfig<'static> {
    // Official container image for SurrealDB
    // https://hub.docker.com/r/surrealdb/surrealdb/tags
    ContainerConfig::new(
        "postgresdb",
        "postgres",
        "16.3-bookworm",
        "0.0.0.0",
        5432,
        None,
        Some(&["POSTGRES_PASSWORD=postgres"]),
        None,
        true, // Keep the container running for re-use
        true, // Keep the same container config across all env. setups.
        WaitStrategy::WaitUntilConsoleOutputContains(
            "database system is ready to accept connections".to_string(),
            30,
        ),
    )
}
