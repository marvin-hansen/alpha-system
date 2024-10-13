mod getters;

use common_database::prelude::ClickHouseConfig;
use common_env::prelude::EnvironmentType;
use db_specs_clickhouse::clickhouse;

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct ClickhouseConfigManager {
    dbg: bool,
    db_clickhouse_config: ClickHouseConfig,
}

impl ClickhouseConfigManager {
    pub fn new(env_type: &EnvironmentType) -> Self {
        Self::build(false, env_type)
    }

    pub fn with_debug(env_type: &EnvironmentType) -> Self {
        Self::build(true, env_type)
    }

    fn build(dbg: bool, env_type: &EnvironmentType) -> ClickhouseConfigManager {
        ClickhouseConfigManager {
            dbg,
            db_clickhouse_config: get_clickhouse_config(dbg, env_type),
        }
    }
}
fn get_clickhouse_config(dbg: bool, env_type: &EnvironmentType) -> ClickHouseConfig {
    if dbg {
        println!("[ClickhouseConfigManager]: get_clickhouse_config");
    }
    clickhouse::get_clickhouse_config(env_type)
}

impl ClickhouseConfigManager {
    pub fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[ClickhouseConfigManager]: {}", msg);
        }
    }
}
