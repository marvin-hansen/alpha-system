use postgres_test_utils::prelude::*;

#[tokio::test]
async fn test_postgres_full_teardown() {
    let result = postgres_full_teardown(DB_TEST_URL).await;
    //dbg!(&result);
    assert!(result.is_ok());
}
