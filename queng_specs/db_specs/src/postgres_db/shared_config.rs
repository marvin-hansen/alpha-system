use common_database::prelude::PostgresDBConfig;

pub(crate) fn get_base_postgres_db_config() -> PostgresDBConfig {
    PostgresDBConfig::new(
        "localhost".to_string(),
        "postgres".to_string(),
        "postgres".to_string(),
        "test".to_string(),
        5432,
        5,
    )
}
