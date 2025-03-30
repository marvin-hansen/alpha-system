/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

mod import_portfolio_configs;

use common_exchange::PortfolioConfig;
use environment_manager::EnvironmentManager;
use pg_cmdb_manager::PostgresCMDBManager;
use portfolio_specs::get_all_portfolio_specs;
use postgres_config_manager::PostgresConfigManager;

#[derive(Debug, Clone)]
pub struct ConfigImportManager {
    dbg: bool,
    dbm: PostgresCMDBManager,
    portfolio_configs: Vec<PortfolioConfig>,
}

impl ConfigImportManager {
    pub async fn new() -> Self {
        Self::build(false, false).await
    }

    pub async fn with_debug() -> Self {
        Self::build(true, false).await
    }

    pub async fn with_test_and_debug() -> Self {
        Self::build(true, true).await
    }

    async fn build(dbg: bool, test: bool) -> Self {
        let env_manager = EnvironmentManager::new();
        let env_type = env_manager.env_type();
        if dbg {
            println!("[main]: Environment type: {env_type:?}");
        }

        let pg_cfg_manager = PostgresConfigManager::new(&env_type);
        let dsn = pg_cfg_manager.pg_connection_url();

        let dbm = if test {
            PostgresCMDBManager::with_test_and_debug(&dsn)
                .await
                .expect("Failed to create PostgresSMDBManager")
        } else {
            PostgresCMDBManager::new(&dsn)
                .await
                .expect("Failed to create PostgresSMDBManager")
        };

        let portfolio_configs = get_all_portfolio_specs();

        Self {
            dbg,
            dbm,
            portfolio_configs,
        }
    }

    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ConfigImportManager]: {s}");
        }
    }
}
