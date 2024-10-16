use crate::PostgresConfigManager;
use common_database::prelude::PostgresDBConfig;

impl PostgresConfigManager {
    pub fn postgres_db_config(&self) -> &PostgresDBConfig {
        &self.db_postgres_config
    }

    pub fn pg_connection_url(&self) -> String {
        self.db_postgres_config.pg_connection_url()
    }
}
