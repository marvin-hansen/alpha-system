use environment_manager::EnvironmentManager;
use mimalloc::MiMalloc;
use postgres_config_manager::PostgresConfigManager;
use postgres_utils::PostgresUtil;
use std::error::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dbg_print("Setup autoconfiguration");
    let config_manager = EnvironmentManager::new();
    let env_type = config_manager.env_type();
    println!("[main]: Environment type: {:?}", env_type);

    dbg_print("Configure postgres database manager");
    let pg_cfg_manager = PostgresConfigManager::new(&env_type);
    let dsn = pg_cfg_manager.pg_connection_url();

    dbg_print("Configure postgres util");
    let pg_util = PostgresUtil::new(&dsn)
        .await
        .expect("Failed to create PostgresUtil Util");

    dbg_print("Setup postgres database");
    pg_util
        .setup_all_db()
        .await
        .expect("Failed to setup all databases");

    Ok(())
}

fn dbg_print(msg: &str) {
    if DBG {
        println!("[main]: {}", msg)
    }
}
