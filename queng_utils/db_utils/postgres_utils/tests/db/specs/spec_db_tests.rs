use env_utils::EnvUtil;
use postgres_utils::PostgresUtil;
use std::env;

// Starts a k
async fn setup_ci_env() {
    // Set the environment variable.
    env::set_var("ENV", "CI");

    // Create new Env Utils
    let mut ci_env = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    // Setup Postgres DB container for CI tests to run
    ci_env
        .setup_container_postgres_db()
        .await
        .expect("Failed to setup ci api proxy container");
}

#[tokio::test]
async fn test_spec_db() {
    // Setup CI environment
    setup_ci_env().await;

    // Create PGUtils
    let dsn =
        "host=localhost user=postgres password=postgres dbname=postgres port=5432 sslmode=disable";
    let res = PostgresUtil::new(dsn).await;
    assert!(res.is_ok());
    let pg_utils = res.unwrap();

    pg_utils.close().await
}
