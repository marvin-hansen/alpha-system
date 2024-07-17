use env_utils::EnvUtil;
use postgres_utils::PostgresUtil;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

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

    // Test create DB
    let result = pg_utils.specs.create_spec_db().await;
    assert!(result.is_ok()); // Check if the operation was successful

    // Test verify DB
    let res = pg_utils.specs.verify_spec_db_exists().await;
    assert!(res.is_ok());

    let db_created = res.unwrap();
    assert!(db_created);

    // Test delete DB
    // let res = pg_utils.specs.drop_spec_db().await;
    // assert!(res.is_ok());

    sleep(Duration::from_millis(100)).await;
    pg_utils.close().await
}
