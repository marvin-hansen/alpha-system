use common_config::ServiceConfig;
use environment_manager::EnvironmentManager;
use pg_smdb_manager::PostgresSMDBManager;
use postgres_config_manager::PostgresConfigManager;
use service_specs_all::prelude as service_specs;

mod import_services;

#[derive(Debug, Clone)]
pub struct ServiceImportManager {
    dbg: bool,
    dbm: PostgresSMDBManager,
    services: Vec<ServiceConfig>,
}

impl ServiceImportManager {
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
            println!("[main]: Environment type: {:?}", env_type);
        }

        let pg_cfg_manager = PostgresConfigManager::new(&env_type);
        let dsn = pg_cfg_manager.pg_connection_url();

        let dbm = if test {
            PostgresSMDBManager::with_test_and_debug(&dsn)
                .await
                .expect("Failed to create PostgresSMDBManager")
        } else {
            PostgresSMDBManager::new(&dsn)
                .await
                .expect("Failed to create PostgresSMDBManager")
        };

        let services = service_specs::get_all_service_specs();

        Self { dbg, dbm, services }
    }

    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ServiceImportManager]: {}", s);
        }
    }
}
