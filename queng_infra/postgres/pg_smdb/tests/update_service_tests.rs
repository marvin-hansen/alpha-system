use common_config::prelude::{ServiceConfig, ServiceID};
use diesel::Connection;
use pg_smdb::model::service;
use postgres_test_utils::prelude::*;

#[tokio::test]
async fn test_service_update() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let service_config = get_test_service_config();
    let result = service::Service::create(conn, &service_config);
    // dbg!(&result);
    assert!(result.is_ok());

    let param_service_id = ServiceID::SMDB;
    let result = service::Service::check_if_service_id_exists(conn, param_service_id);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());

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
    assert!(result.is_ok());

    let service = result.unwrap();
    assert_eq!(service.name(), "new_name");
    assert_eq!(service.version(), 2);
    assert_eq!(service.online(), true);
    assert_eq!(service.description(), "description");
    assert_eq!(service.health_check_uri(), "health_check_uri");
    assert_eq!(service.base_uri(), "base_uri");
    assert_eq!(service.dependencies(), &vec![ServiceID::DBGW]);
    assert_eq!(
        service.endpoints(),
        &vec![
            common_config::prelude::Endpoint::default(),
            common_config::prelude::Endpoint::default(),
        ]
    );
}
