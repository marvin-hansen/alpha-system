use common_ims::prelude::IntegrationConfig;
use environment_manager::EnvironmentManager;
use pg_imdb_manager::PostgresIMDBManager;
use postgres_config_manager::PostgresConfigManager;

mod import_integration_configs;

#[derive(Debug, Clone)]
pub struct IntegrationImportManager {
    dbg: bool,
    dbm: PostgresIMDBManager,
    integration_configs: Vec<IntegrationConfig>,
}
impl IntegrationImportManager {
    pub async fn new() -> Self {
        Self::build(false, false).await
    }

    pub async fn with_debug() -> Self {
        Self::build(true, false).await
    }

    pub async fn with_test_and_debug() -> Self {
        Self::build(true, true).await
    }
}

impl IntegrationImportManager {
    async fn build(dbg: bool, test: bool) -> Self {
        let env_manager = EnvironmentManager::new();
        let env_type = env_manager.env_type();
        if dbg {
            println!("[main]: Environment type: {:?}", env_type);
        }

        let pg_cfg_manager = PostgresConfigManager::new(&env_type);
        let dsn = pg_cfg_manager.pg_connection_url();

        let dbm = if test {
            PostgresIMDBManager::with_test_and_debug(&dsn)
                .await
                .expect("Failed to create PostgresIMDBManager")
        } else {
            PostgresIMDBManager::new(&dsn)
                .await
                .expect("Failed to create PostgresIMDBManager")
        };

        let integration_configs = integration_specs_all::get_all_integration_configs();

        Self {
            dbg,
            dbm,
            integration_configs,
        }
    }

    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[IntegrationImportManager]: {}", s);
        }
    }
}
