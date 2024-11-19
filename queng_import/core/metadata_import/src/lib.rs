use environment_manager::EnvironmentManager;
use pg_mddb_manager::PostgresMDDBManager;
use postgres_config_manager::PostgresConfigManager;

mod import_metadata;

const AUTO_DETECT_PROXY: bool = true;

#[derive(Debug, Clone)]
pub struct MetadataImportManager {
    dbg: bool,
    dbm: PostgresMDDBManager,
}

impl MetadataImportManager {
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
            PostgresMDDBManager::with_test_and_debug(&dsn)
                .await
                .expect("Failed to create PostgresMDDBManager")
        } else {
            PostgresMDDBManager::new(&dsn)
                .await
                .expect("Failed to create PostgresMDDBManager")
        };

        Self { dbg, dbm }
    }

    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[MetadataImportManager]: {s}");
        }
    }
}
