use common_database::PostgresDBConfig;
use common_env::EnvironmentType;

#[must_use]
pub fn get_postgres_config(env_type: &EnvironmentType) -> PostgresDBConfig {
    match env_type {
        EnvironmentType::LOCAL => get_base_postgres_db_config(),
        EnvironmentType::CI => get_base_postgres_db_config(),
        _ => panic!("Environment not supported"),
    }
}

#[must_use]
pub fn get_cluster_db_host() -> String {
    // Note, there are three services: postgres-cluster-rw,postgres-cluster-r, and postgres-cluster-ro
    // Select the postgres-cluster-rw service if you want to write to the database. The others are read and read-only.

    "postgres-rw.default.svc.cluster.local".to_string()
}

#[must_use]
pub const fn get_cluster_db_config(
    pg_user: String,
    pg_password: String,
    pg_database: String,
    pg_host: String,
) -> PostgresDBConfig {
    PostgresDBConfig::new(pg_host, pg_user, pg_password, pg_database, 5432, 10)
}

fn get_base_postgres_db_config() -> PostgresDBConfig {
    PostgresDBConfig::new(
        "localhost".to_string(),
        "postgres".to_string(),
        "postgres".to_string(),
        "postgres".to_string(),
        5432,
        5,
    )
}
