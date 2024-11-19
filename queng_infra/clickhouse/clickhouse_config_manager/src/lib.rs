mod getters;

use common_database::ClickHouseConfig;
use common_env::EnvironmentType;

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct ClickhouseConfigManager {
    dbg: bool,
    db_clickhouse_config: ClickHouseConfig,
}

impl ClickhouseConfigManager {
    #[must_use]
    pub fn new(env_type: &EnvironmentType) -> Self {
        Self::build(false, env_type)
    }

    #[must_use]
    pub fn with_debug(env_type: &EnvironmentType) -> Self {
        Self::build(true, env_type)
    }

    fn build(dbg: bool, env_type: &EnvironmentType) -> Self {
        Self {
            dbg,
            db_clickhouse_config: get_clickhouse_config(dbg, env_type),
        }
    }
}
fn get_clickhouse_config(dbg: bool, env_type: &EnvironmentType) -> ClickHouseConfig {
    if dbg {
        println!("[ClickhouseConfigManager]: get_clickhouse_config");
    }
    db_specs_clickhouse::get_clickhouse_config(env_type)
}

impl ClickhouseConfigManager {
    pub fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[ClickhouseConfigManager]: {msg}");
        }
    }
}
