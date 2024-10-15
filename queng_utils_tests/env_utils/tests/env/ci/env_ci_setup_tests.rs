use env_utils::EnvUtil;

#[tokio::test]
async fn test_env_util_setup_ci() {
    let ci_env = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");
    let docker_util = &mut ci_env.docker_util();

    // Initial setup of the CI test environment
    ci_env.setup_ci().await.expect("Failed to setup ci env");

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

    let postgres_container_name = ci_env.postgres_db_container_name();
    let exists = docker_util
        .check_if_container_is_running(&postgres_container_name)
        .expect("Failed to check if postgres_ container exists");
    assert!(exists);

    println!("✅ OK: Container: {} created", postgres_container_name);
    println!();
}
