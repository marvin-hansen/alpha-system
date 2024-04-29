use env_utils::EnvUtil;
use std::thread::sleep;
use std::time::Duration;

#[tokio::test]
async fn test_env_util_setup_ci() {
    // Initial setup of the CI test environment
    let mut ci_env = EnvUtil::with_debug();
    ci_env.setup_ci().await.expect("Failed to setup test env");

    // Verify that the container was created
    let docker_util = &mut ci_env.get_docker_util().expect("Failed to get docker util");
    let container_name = ci_env.clickhouse_container_name();
    let exists = docker_util
        .check_if_container_exists(&container_name)
        .expect("Failed to check if container exists");
    assert!(exists);

    println!("✅ OK: Container name: {} created", container_name);
    println!();

    // At a later stage, containers will be re-used or re-created
    // depending on the container configuration
    let mut test_env = EnvUtil::with_debug();

    test_env.setup_ci().await.expect("Failed to setup test env");

    // Verify that the container was re-used
    let container_name = test_env.clickhouse_container_name();
    let exists = docker_util
        .check_if_container_exists(&container_name)
        .expect("Failed to check if container exists");
    assert!(exists);

    println!("✅ OK: Container name: {} re-used", container_name);
    println!();

    // Give some extra time
    sleep(Duration::from_millis(100));

    // Teardown of the CI test environment
    test_env
        .teardown_ci()
        .await
        .expect("Failed to teardown test env");

    // Verify that the container was deleted
    let exists = docker_util
        .check_if_container_exists(&container_name)
        .expect("Failed to check if container exists");
    assert!(!exists);

    println!("✅ OK: Container name: {} deleted", container_name);
    println!();

    println!("All tests passed:");
    println!("  ✅ OK: TestEnv CI created");
    println!("  ✅ OK: TestEnv CI re-used");
    println!("  ✅ OK: TestEnv CI stopped");
}
