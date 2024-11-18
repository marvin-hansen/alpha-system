use container_specs_postgres::postgres_db_container_config;
use docker_utils::DockerUtil;

#[tokio::test]
async fn all_setup() {
    let env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Start or reuse a test postgres container
    let container_config = postgres_db_container_config();
    let result = env.get_or_start_container_config(&container_config);
    // dbg!(&result);
    assert!(result.is_ok());
}
