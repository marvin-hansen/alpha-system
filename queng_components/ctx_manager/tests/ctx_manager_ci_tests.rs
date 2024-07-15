use std::env;

use common_env::prelude::EnvironmentType;
use ctx_manager::CtxManager;

fn setup() {
    env::set_var("ENV", "CI");
}

#[test]
fn test_new() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CI);
}

#[test]
fn test_env_type() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CI);
}

#[test]
fn test_display() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CI);
    assert_eq!(ctm.to_string(), "CtxManager { env_type: CI }");
}
