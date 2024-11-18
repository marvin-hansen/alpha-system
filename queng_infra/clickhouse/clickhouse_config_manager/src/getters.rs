use crate::ClickhouseConfigManager;
use common_database::ClickHouseConfig;

impl ClickhouseConfigManager {
    pub fn clickhouse_db_config(&self) -> &ClickHouseConfig {
        &self.db_clickhouse_config
    }
}
