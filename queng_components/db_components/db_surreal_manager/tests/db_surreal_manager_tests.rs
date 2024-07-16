use common_database::prelude::SurrealDBConfig;
use db_surreal_manager::SurrealDBManager;
use env_utils::EnvUtil;
use std::env;

async fn setup_env() {
    env::set_var("ENV", "CI");
    let mut env = EnvUtil::new().await.expect("Failed to get EnvUtils");

    env.setup_container_surreal_db()
        .await
        .expect("Failed to setup SurrealDB container");
}

#[tokio::test]
async fn test_new() {
    setup_env().await;

    let db_config = SurrealDBConfig::default();

    let res = SurrealDBManager::new(&db_config).await;

    assert!(res.is_ok());

    let dbm = res.unwrap();

    assert!(dbm.is_healthy().await.is_ok())
}
