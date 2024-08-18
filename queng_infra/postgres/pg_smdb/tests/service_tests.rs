use common_config::prelude::ServiceID::SMDB;
use common_config::prelude::{ServiceConfig, ServiceID};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use pg_smdb::model::service;
use pg_smdb::model::service::UpdateService;
use pg_smdb::run_smdb_db_migration;
use std::env;

fn postgres_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .or_else(|_| env::var("POSTGRES_DATABASE_URL"))
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

fn test_db_migration(conn: &mut pg_smdb::Connection) {
    let res = run_smdb_db_migration(conn);
    //dbg!(&result);
    assert!(res.is_ok());
}

#[test]
fn test_service() {
    let pool = postgres_connection_pool();
    let conn = &mut pool.get().unwrap();

    println!("Test DB migration");
    test_db_migration(conn);

    println!("Test create!");
    test_create_service(conn);

    println!("Test count!");
    test_count_service(conn);

    println!("Test check if exists!");
    test_check_if_service_id_exists(conn);

    println!("Test check if online!");
    test_check_if_service_id_online(conn);

    println!("Test get all online services!");
    test_get_all_online_services(conn);

    println!("Test get all offline services!");
    test_get_all_offline_services(conn);

    println!("Test get all service dependencies!");
    test_get_all_service_dependencies(conn);

    println!("Test get all service endpoints!");
    test_get_all_service_endpoints(conn);

    println!("Test read!");
    test_service_read(conn);

    println!("Test read_all!");
    test_service_read_all(conn);

    println!("Test set service online!");
    test_set_service_online(conn);

    println!("Test set service offline!");
    test_set_service_offline(conn);

    println!("Test update service!");
    test_service_update(conn);

    println!("Test delete service!");
    test_service_delete(conn);
}

fn test_create_service(conn: &mut pg_smdb::Connection) {
    let id = ServiceID::SMDB;
    let name = "name".to_string();
    let version = 1;
    let online = true;
    let description = "description".to_string();
    let health_check_uri = "health_check_uri".to_string();
    let base_uri = "base_uri".to_string();
    let dependencies = vec![ServiceID::DBGW];
    let endpoints = Vec::from([
        common_config::prelude::Endpoint::default(),
        common_config::prelude::Endpoint::default(),
    ]);

    let service_config = ServiceConfig::new(
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

    let result = service::Service::create(conn, &service_config);
    dbg!(&result);
    assert!(result.is_ok());

    let service = result.unwrap();

    assert_eq!(service.svc_id(), &id);
    assert_eq!(service.name(), &name);
    assert_eq!(service.version(), version);
    assert_eq!(service.online(), online);
    assert_eq!(service.description(), &description);
    assert_eq!(service.health_check_uri(), &health_check_uri);
    assert_eq!(service.base_uri(), &base_uri);
    assert_eq!(service.dependencies(), &dependencies);
    assert_eq!(service.endpoints(), &endpoints);
}

fn test_count_service(conn: &mut pg_smdb::Connection) {
    let result = service::Service::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

fn test_check_if_service_id_exists(conn: &mut pg_smdb::Connection) {
    let param_service_id = SMDB;
    let result = service::Service::check_if_service_id_exists(conn, param_service_id);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

fn test_check_if_service_id_online(conn: &mut pg_smdb::Connection) {
    let param_service_id = SMDB;
    let result = service::Service::check_if_service_id_online(conn, param_service_id);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

fn test_get_all_online_services(conn: &mut pg_smdb::Connection) {
    let result = service::Service::get_all_online_services(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap().len() > 0);
}

fn test_get_all_offline_services(conn: &mut pg_smdb::Connection) {
    let result = service::Service::get_all_offline_services(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

fn test_get_all_service_dependencies(conn: &mut pg_smdb::Connection) {
    let service_id = SMDB;

    let result = service::Service::get_all_service_dependencies(conn, service_id);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
}

fn test_get_all_service_endpoints(conn: &mut pg_smdb::Connection) {
    let service_id = SMDB;

    let result = service::Service::get_all_service_endpoints(conn, service_id);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 2);
}

fn test_service_read(conn: &mut pg_smdb::Connection) {
    let service_id = SMDB;

    let result = service::Service::read(conn, service_id);
    // dbg!(&result);
    assert!(result.is_ok());

    let service = result.unwrap();
    assert_eq!(service.svc_id(), &service_id);
    assert_eq!(service.name(), "name");
    assert_eq!(service.version(), 1);
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

fn test_service_read_all(conn: &mut pg_smdb::Connection) {
    let result = service::Service::read_all(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    let services = result.unwrap();
    assert!(services.len() > 0);
}

fn test_set_service_online(conn: &mut pg_smdb::Connection) {
    let service_id = SMDB;

    let result = service::Service::set_service_online(conn, service_id);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = service::Service::check_if_service_id_online(conn, service_id);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

fn test_set_service_offline(conn: &mut pg_smdb::Connection) {
    let service_id = SMDB;

    let result = service::Service::set_service_offline(conn, service_id);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = service::Service::check_if_service_id_online(conn, service_id);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

fn test_service_update(conn: &mut pg_smdb::Connection) {
    // check if service_id exists so we can update the service
    let param_service_id = SMDB;
    let result = service::Service::check_if_service_id_exists(conn, param_service_id);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());

    let update = UpdateService::new(
        Some("new_test".to_string()),
        Some(2),
        Some(true),
        None,
        None,
        None,
        None,
        None,
    );

    let result = service::Service::update(conn, param_service_id, &update);
    // dbg!(&result);
    assert!(result.is_ok());

    let service = result.unwrap();
    assert_eq!(service.name(), "new_test");
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

fn test_service_delete(conn: &mut pg_smdb::Connection) {
    let result = service::Service::read(conn, SMDB);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = service::Service::delete(conn, 1);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = service::Service::read(conn, SMDB);
    // dbg!(&result);
    assert!(result.is_err());

    let result = service::Service::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
