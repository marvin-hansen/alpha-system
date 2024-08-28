use postgres_test_utils::prelude::*;

#[tokio::test]
async fn test_full_setup() {
    let result = postgres_full_setup(DB_TEST_URL).await;
    //dbg!(&result);
    assert!(result.is_ok());
}
