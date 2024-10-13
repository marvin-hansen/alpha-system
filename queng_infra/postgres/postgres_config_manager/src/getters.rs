use crate::PostgresConfigManager;
use common_database::prelude::PostgresDBConfig;

impl PostgresConfigManager {
    pub fn postgres_db_config(&self) -> &PostgresDBConfig {
        &self.db_postgres_config
    }
}
