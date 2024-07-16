use common_config::prelude::ContainerConfig;

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
        1,    // Wait a few second until the container finished starting up.
    )
}
