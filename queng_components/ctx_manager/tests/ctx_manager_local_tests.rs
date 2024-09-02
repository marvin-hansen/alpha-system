use std::env;

use common_env::prelude::EnvironmentType;
use ctx_manager::CtxManager;

fn setup() {
    env::set_var("ENV", "LOCAL");
}

#[tokio::test]
async fn test_new() {
    setup();

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);
}

#[tokio::test]
async fn test_env_type() {
    setup();

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);
}

#[tokio::test]
async fn test_display() {
    setup();

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);
    assert_eq!(ctm.to_string(), "CtxManager { env_type: LOCAL }");
}
