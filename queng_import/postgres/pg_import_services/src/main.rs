mod print_utils;

use environment_manager::EnvironmentManager;
use mimalloc::MiMalloc;
use pg_smdb_manager::PostgresSMDBManager;
use postgres_config_manager::PostgresConfigManager;
use postgres_utils::PostgresUtil;
use service_specs_all::prelude as service_specs;
use std::error::Error;
use std::process::exit;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print_utils::print_start_header();

    dbg_print("Setup autoconfiguration");
    let config_manager = EnvironmentManager::new();
    let env_type = config_manager.env_type();
    println!("[main]: Environment type: {:?}", env_type);

    dbg_print("Configure postgres database manager");
    let pg_cfg_manager = PostgresConfigManager::new(&env_type);
    let dsn = pg_cfg_manager.pg_connection_url();

    let dbm_smdb = PostgresSMDBManager::new(&dsn)
        .await
        .expect("Failed to create PostgresSMDBManager");

    dbg_print("Configure postgres util");
    let pg_util = PostgresUtil::new(&dsn)
        .await
        .expect("Failed to create PostgresUtil Util");

    dbg_print("Setup postgres database");
    pg_util
        .setup_all_db()
        .await
        .expect("Failed to setup all databases");

    let all_services = service_specs::get_all_service_specs();
    let expected_count = all_services.len();
    // Count if there is any service already in the database
    let actual_count = pg_util
        .count_services()
        .await
        .expect("Failed to count services") as usize;

    // If all services have already been imported, exit the program
    if actual_count == expected_count {
        print_utils::print_already_header();
        exit(0);
    }

    dbg_print("Setup all postgres databases");
    pg_util
        .setup_all_db()
        .await
        .expect("Failed to setup all DBs");

    // If nothing has been imported yet, import all services
    if actual_count == 0 {
        dbg_print("Import sll services");
        pg_util
            .import_service_collection(&all_services)
            .await
            .expect("Failed to import services");

        // Count all imported services
        let post_import_count = pg_util
            .count_services()
            .await
            .expect("Failed to count services") as usize;

        // Check if all services have been imported
        if post_import_count != expected_count {
            dbg_print("Failed to import all services. Check database records manually to determine missing services");
            exit(42);
        }

        print_utils::print_stop_header(post_import_count, true);
    }

    // If some services have been already imported, yet some new new have been added,
    // so we have to check one by one to test which one to add.
    if actual_count > 0 && actual_count < expected_count {
        let delta_count = actual_count - expected_count;

        for s in &all_services {
            let id = s.svc_id();
            let exists = dbm_smdb
                .check_if_service_id_exists(id)
                .await
                .expect("Failed to check if service exists");

            if !exists {
                dbg_print(&format!("Importing service: {}", id));
                dbm_smdb
                    .insert_service(s)
                    .await
                    .expect("Failed to import service");
            }
        }

        print_utils::print_stop_header(delta_count, true);
    }

    Ok(())
}

fn dbg_print(msg: &str) {
    if DBG {
        println!("[main]: {}", msg)
    }
}
