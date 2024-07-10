use clickhouse_utils::ClickhouseUtil;
use env_utils::EnvUtil;

use std::{env, time};
use tokio::time::sleep;

async fn setup_ci_env() {
    // Set the environment variable.
    env::set_var("ENV", "CI");

    // Create new Env Utils
    let mut ci_env = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    // Initiate CI container
    ci_env
        .setup_container_clickhouse()
        .await
        .expect("Failed to setup clickhouse container");
}

#[tokio::test]
async fn setup_db_test() {
    setup_ci_env().await;

    let dsn = "127.0.0.1:9000".to_string();
    let result = ClickhouseUtil::new(dsn).await;

    assert!(result.is_ok());

    let ch_utils = result.unwrap();

    let setup_result = ch_utils.setup_all_db().await;
    assert!(setup_result.is_ok());

    let wait_time = time::Duration::from_millis(500);
    sleep(wait_time).await;

    let teardown_result = ch_utils.teardown_all_db().await;
    assert!(teardown_result.is_ok());
}
