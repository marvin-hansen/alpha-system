use common_config::prelude::{ServiceConfig, ServiceID};
use common_database::prelude::PostgresDBSchema;
use diesel::Connection;
use pg_smdb::model::service;
use postgres_test_utils::prelude::*;

#[tokio::test]
async fn test_service_update_error() {
    postgres_schema_setup(PostgresDBSchema::SMDB, DB_TEST_URL)
        .await
        .expect("FAILED  to setup CMDB schema");

    let mut connection = postgres_connection(DB_TEST_URL).await;
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    // Double check the service isn't in the database
    let param_service_id = ServiceID::SMDB;
    let result = service::Service::check_if_service_id_exists(conn, param_service_id);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(!result.unwrap());

    let id = ServiceID::SMDB;
    let name = "new_name".to_string();
    let version = 2;
    let online = true;
    let description = "description".to_string();
    let health_check_uri = "health_check_uri".to_string();
    let base_uri = "base_uri".to_string();
    let dependencies = vec![ServiceID::DBGW];
    let endpoints = Vec::from([
        common_config::prelude::Endpoint::default(),
        common_config::prelude::Endpoint::default(),
    ]);

    let update = ServiceConfig::new(
        id.clone(),
        name.clone(),
        version.clone(),
        online.clone(),
        description.clone(),
        health_check_uri.clone(),
        base_uri.clone(),
        dependencies.clone(),
        endpoints.clone(),
    );
    let result = service::Service::update(conn, &param_service_id, &update);
    // dbg!(&result);
    assert!(result.is_err());
}
