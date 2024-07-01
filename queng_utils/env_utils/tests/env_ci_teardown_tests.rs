use env_utils::EnvUtil;

#[tokio::test]
async fn test_env_util_teardown_ci() {
    // Initial setup of the CI test environment
    let ci_env = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");
    ci_env
        .teardown_ci()
        .await
        .expect("Failed to setup test env");

    // Verify that the container was created
    let docker_util = &mut ci_env.docker_util();
    let container_name = ci_env.clickhouse_container_name();
    let exists = docker_util
        .check_if_container_exists(container_name)
        .expect("Failed to check if container exists");
    assert!(!exists);

    println!("✅ OK: Container name: {} removed", container_name);
    println!();
}
