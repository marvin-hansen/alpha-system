use common_config::prelude::ServiceID;
use common_pg_queries::service_query::{
    build_check_if_service_id_exists_query, build_check_if_service_id_online_query,
    build_delete_service_query, build_read_all_services_query, build_read_service_by_id_query,
    build_set_svc_online_query, generate_count_table_query,
};

#[test]
fn test_generate_count_table_query() {
    let schema_name = "public";
    let table_name = "test_table";
    let actual_query = generate_count_table_query(schema_name, table_name);
    let expected_query = "SELECT COUNT(*) FROM public.test_table;".to_string();
    assert_eq!(actual_query, expected_query);
}

#[test]
fn test_build_check_if_service_id_exists_query() {
    let id = ServiceID::SMDB;
    let actual_query = build_check_if_service_id_exists_query(&id);
    let expected_query = "SELECT EXISTS (\n        SELECT\n            id\n        FROM\n            system.service\n        WHERE\n            id=1\n        )".to_string();
    assert_eq!(actual_query, expected_query);
}

#[test]
fn test_build_check_if_service_id_online_query() {
    let id = ServiceID::SMDB;
    let actual_query = build_check_if_service_id_online_query(&id);
    let expected_query = "SELECT EXISTS (\n        SELECT\n            id, online\n        FROM\n            system.service\n        WHERE\n            id=1\n        AND\n            online=true\n        )".to_string();
    assert_eq!(actual_query, expected_query);
}

#[test]
fn test_build_set_svc_online_query() {
    let id = ServiceID::SMDB;
    let online = true;
    let actual_query = build_set_svc_online_query(&id, online);
    let expected_query = "UPDATE\n            system.service\n        SET\n            online=true\n        WHERE\n            id=1\n        RETURNING service.online".to_string();
    assert_eq!(actual_query, expected_query);
}

#[test]
fn test_build_read_service_by_id_query() {
    let id = ServiceID::SMDB;
    let actual_query = build_read_service_by_id_query(&id);
    let expected_query = "SELECT\n                id, name, version, online, description, health_check_uri, base_uri, dependencies, exposure,\n                endpoint_name, endpoint_version, endpoint_base_uri, endpoint_port, endpoint_protocol,\n                metric_uri, metric_host, metric_port\n            FROM\n                system.service\n            WHERE\n                id=1".to_string();
    assert_eq!(actual_query, expected_query);
}

#[test]
fn test_build_read_all_services_query() {
    let actual_query = build_read_all_services_query();
    let expected_query = "SELECT\n             id, name, version, online, description, health_check_uri, base_uri, dependencies, exposure,\n             endpoint_name, endpoint_version, endpoint_base_uri, endpoint_port, endpoint_protocol,\n             metric_uri, metric_host, metric_port\n         FROM\n           system.service\n         ORDER BY\n            id".to_string();
    assert_eq!(actual_query, expected_query);
}

#[test]
fn test_build_delete_service_query() {
    let id = ServiceID::SMDB;
    let actual_query = build_delete_service_query(&id);
    let expected_query = "DELETE FROM system.service
             WHERE
                id=1"
        .to_string();
    assert_eq!(actual_query, expected_query);
}
