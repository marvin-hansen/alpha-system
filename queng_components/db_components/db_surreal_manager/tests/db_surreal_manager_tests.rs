use common::prelude::SurrealDBConfig;
use db_surreal_manager::SurrealDBManager;
use std::env;

async fn setup_env() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");
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
