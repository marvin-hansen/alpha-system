/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::ClickhouseConfigManager;
use common_database::ClickHouseConfig;

impl ClickhouseConfigManager {
    #[must_use]
    pub const fn clickhouse_db_config(&self) -> &ClickHouseConfig {
        &self.db_clickhouse_config
    }
}
