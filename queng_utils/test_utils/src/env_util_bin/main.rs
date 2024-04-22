use test_utils::prelude::EnvUtil;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initial setup of the CI test environment
    let test_env = EnvUtil::setup_ci().await.expect("Failed to setup test env");
    let docker_util = &mut test_env.docker_util();

    // Verify that the container was created
    let container_name = test_env.clickhouse_container_name();
    let exists = docker_util
        .check_if_container_exists(&container_name)
        .expect("Failed to check if container exists");
    assert!(exists);

    println!("✅ OK: Container name: {} created", container_name);
    println!();

    // At a later stage, containers will be re-used or re-created
    // depending on the container configuration
    let test_env = EnvUtil::setup_ci().await.expect("Failed to setup test env");

    // Verify that the container was re-used
    let container_name = test_env.clickhouse_container_name();
    let exists = docker_util
        .check_if_container_exists(&container_name)
        .expect("Failed to check if container exists");
    assert!(exists);

    println!("✅ OK: Container name: {} re-used", container_name);
    println!();

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

    Ok(())
}
