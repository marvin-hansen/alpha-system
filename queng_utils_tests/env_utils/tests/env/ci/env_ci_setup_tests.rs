use env_utils::EnvUtil;

#[tokio::test]
async fn test_env_util_setup_ci() {
    // Initial setup of the CI test environment
    let ci_env = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");
    ci_env
        .setup_all_containers()
        .await
        .expect("Failed to setup ci env");

    // Verify that the container was created
    let docker_util = &mut ci_env.docker_util();
    let clickhouse_container_name = ci_env.clickhouse_container_name();
    let exists = docker_util
        .check_if_container_is_running(&clickhouse_container_name)
        .expect("Failed to check if clickhouse container exists");
    assert!(exists);

    println!(
        "✅ OK: Container name: {} created",
        clickhouse_container_name
    );
    println!();

    let api_proxy_container_name = ci_env.api_proxy_container_name();
    let exists = docker_util
        .check_if_container_is_running(&api_proxy_container_name)
        .expect("Failed to check if api_proxy container exists");
    assert!(exists);

    println!(
        "✅ OK: Container name: {} created",
        api_proxy_container_name
    );
    println!();

    ci_env.setup_ci().await.expect("Failed to setup ci env");
}
