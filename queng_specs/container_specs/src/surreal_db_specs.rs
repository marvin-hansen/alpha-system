use common_config::prelude::ContainerConfig;

pub fn surreal_db_container_config() -> ContainerConfig<'static> {
    // Official container image for SurrealDB
    // https://hub.docker.com/r/surrealdb/surrealdb/tags
    ContainerConfig::new(
        "surrealdb",
        "surrealdb/surrealdb",
        "v1.5.4",
        "0.0.0.0",
        8000,
        None,
        None,
        // Enables authentication and a default user via start command passed to Docker.
        // Also, the argument order is fixed meaning you cannot remove the log level,
        // otherwise the container fails to start.
        // https://surrealdb.com/docs/surrealdb/installation/running/docker
        Some(&[
            "start", "--log", "warning", "--auth", "--user", "root", "--pass", "root",
        ]),
        true, // Keep the container running for re-use
        true, // Keep the same container config across all env. setups.
        1,    // Wait a few second until the container finished starting up.
    )
}
