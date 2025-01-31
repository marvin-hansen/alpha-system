/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::PostgresConfigManager;
use common_database::PostgresDBConfig;

impl PostgresConfigManager {
    #[must_use]
    pub const fn postgres_db_config(&self) -> &PostgresDBConfig {
        &self.db_postgres_config
    }

    #[must_use]
    pub fn pg_connection_url(&self) -> String {
        self.db_postgres_config.pg_connection_url()
    }
}
