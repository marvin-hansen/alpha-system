use clickhouse_utils::ClickhouseUtil;

use container_specs::clickhouse_container_specs::clickhouse_container_config;
use docker_utils::DockerUtil;
use std::{env, time};
use tokio::time::sleep;

async fn setup_ci_env() {
    // Set the environment variable.
    env::set_var("ENV", "CI");

    // Create new DockerUtil
    let ci_env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Initiate CI container
    let container_config = clickhouse_container_config();
    ci_env
        .setup_container(&container_config)
        .await
        .expect("Failed to setup ci api proxy container");
}

#[tokio::test]
async fn setup_db_test() {
    setup_ci_env().await;

    let dsn = "127.0.0.1:9000";
    let result = ClickhouseUtil::new(dsn).await;

    assert!(result.is_ok());

    let ch_utils = result.unwrap();

    let setup_result = ch_utils.setup_all_db().await;
    assert!(setup_result.is_ok());

    let wait_time = time::Duration::from_millis(500);
    sleep(wait_time).await;

    let teardown_result = ch_utils.teardown_all_db(true).await;
    assert!(teardown_result.is_ok());
}
