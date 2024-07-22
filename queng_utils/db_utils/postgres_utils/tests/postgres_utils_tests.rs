use container_specs::postgres_db_specs::postgres_db_container_config;
use docker_utils::DockerUtil;
use postgres_utils::PostgresUtil;
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

async fn get_client() -> PostgresUtil {
    let dsn = "host=127.0.0.1 user=postgres password=postgres dbname=postgres";
    let res = PostgresUtil::with_debug(dsn).await;
    assert!(res.is_ok());

    res.unwrap()
}

#[tokio::test]
async fn postgres_db_test() {
    //
    setup_ci_env().await;

    let pg_util = get_client().await;

    postgres_db_setup_test(&pg_util).await;

    postgres_db_import_test(&pg_util).await;

    postgres_db_teardown_test(&pg_util).await;

    pg_util.close().await;
}
async fn postgres_db_setup_test(util: &PostgresUtil) {
    let res = util.setup_all_db().await;
    assert!(res.is_ok());
}

async fn postgres_db_import_test(util: &PostgresUtil) {
    let svc = smdb_specs::smdb_service_config();
    let res = util.specs.insert_service(&svc).await;
    assert!(res.is_ok());
}

async fn postgres_db_teardown_test(util: &PostgresUtil) {
    let res = util.teardown_all_db(false).await;
    assert!(res.is_ok());

    let res = util.drop_all_db().await;
    assert!(res.is_ok());
}
