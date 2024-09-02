use std::env;

use common_env::prelude::EnvironmentType;
use ctx_manager::CtxManager;

fn setup() {
    env::set_var("ENV", "CLUSTER");
}

#[tokio::test]
async fn test_new() {
    setup();

    let ctm = CtxManager::new().await;

    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
}

#[tokio::test]
async fn test_env_type() {
    setup();

    let ctm = CtxManager::new().await;

    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
}

#[tokio::test]
async fn test_display() {
    setup();

    let ctm = CtxManager::new().await;

    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);

    assert_eq!(ctm.to_string(), "CtxManager { env_type: CLUSTER }");
}
