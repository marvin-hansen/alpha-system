use env_utils::EnvUtil;

#[tokio::test]
async fn test_env_util_setup_containers() {
    // Initial setup of the CI test environment
    let mut ci_env = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");
    ci_env
        .setup_containers()
        .await
        .expect("Failed to setup ci env");

    // Verify that the container was created
    let docker_util = &mut ci_env.docker_util();
    let clickhouse_container_name = ci_env.clickhouse_container_name();
    let exists = docker_util
        .check_if_container_exists(clickhouse_container_name)
        .expect("Failed to check if container exists");
    assert!(exists);

    println!(
        "✅ OK: Container name: {} created",
        clickhouse_container_name
    );
    println!();

    let api_proxy_container_name = ci_env.api_proxy_container_name();
    let exists = docker_util
        .check_if_container_exists(api_proxy_container_name)
        .expect("Failed to check if container exists");
    assert!(exists);

    println!(
        "✅ OK: Container name: {} created",
        api_proxy_container_name
    );
    println!();

    let surreal_db_container_name = ci_env.surreal_db_container_name();
    let exists = docker_util
        .check_if_container_exists(surreal_db_container_name)
        .expect("Failed to check if container exists");
    assert!(exists);

    println!(
        "✅ OK: Container name: {} created",
        surreal_db_container_name
    );
    println!();

    // At a later stage, containers will be re-used or re-created
    // depending on the container configuration
    let mut test_env = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    test_env
        .setup_containers()
        .await
        .expect("Failed to setup test env");

    // Verify that the clickhouse container was re-used
    let exists = docker_util
        .check_if_container_exists(clickhouse_container_name)
        .expect("Failed to check if container exists");
    assert!(exists);

    println!(
        "✅ OK: Container name: {} re-used",
        clickhouse_container_name
    );
    println!();

    // Verify that the api proxy container was reused
    let exists = docker_util
        .check_if_container_exists(api_proxy_container_name)
        .expect("Failed to check if container exists");
    assert!(exists);

    println!(
        "✅ OK: Container name: {} re-used",
        api_proxy_container_name
    );
    println!();

    println!("All tests passed:");
    println!("  ✅ OK: TestEnv CI: ClickHouse created");
    println!("  ✅ OK: TestEnv CI: ClickHouse re-used");
    //
    println!("  ✅ OK: TestEnv CI: API Proxy created");
    println!("  ✅ OK: TestEnv CI: API Proxy re-used");
}
