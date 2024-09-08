use diesel::Connection;
use docker_utils::prelude::DockerUtil;
use postgres_migrations::prelude::{get_or_wait_for_postgres_connection, DB_TEST_URL};

//
// Somehow tests seem to be executed or sorted in alphabetical order,
// so make sure that the setup is on top of the stack.
#[tokio::test]
async fn all_setup() {
    let env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Start or reuse a test postgres container
    let result = env.setup_container_postgres_db().await;
    // dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_schema_migration() {
    // Create a new connection
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());
}
