use std::env;

use db_postgres_manager::PostgresDBManager;
use env_utils::EnvUtil;

async fn setup_ci_env() {
    // Set the environment variable.
    env::set_var("ENV", "CI");

    let env_util = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    env_util
        .setup_container_postgres_db()
        .await
        .expect("Failed to setup postgres container");

    env_util
        .setup_postgres()
        .await
        .expect("Failed to setup postgres");
}

#[tokio::test]
async fn test_db_postgres_manager() {
    setup_ci_env().await;

    let pg_config = db_specs::postgres_db::get_ci_db_config();

    let pgm = PostgresDBManager::with_debug(&pg_config)
        .await
        .expect("Failed to get PostgresDBManager");

    pgm.close().await
}
