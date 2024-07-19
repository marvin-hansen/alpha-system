use container_specs::postgres_db_specs::postgres_db_container_config;
use docker_utils::DockerUtil;
use std::env;

async fn setup_ci_env() {
    // Set the environment variable.
    env::set_var("ENV", "CI");

    // Create new DockerUtil
    let ci_env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Initiate CI container
    let container_config = postgres_db_container_config();
    ci_env
        .setup_container(&container_config)
        .await
        .expect("Failed to setup ci api proxy container");
}
#[tokio::test]
async fn postgres_db_test() {
    setup_ci_env().await;
}
