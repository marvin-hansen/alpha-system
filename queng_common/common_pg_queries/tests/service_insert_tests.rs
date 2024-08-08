// use common_config::prelude::ServiceConfig;
// use smdb_specs::smdb_service_config;
//
// fn get_service_config() -> ServiceConfig {
//     smdb_service_config()
// }

// #[test]
// fn test_build_insert_service_query() {
//     let config = get_service_config();
//     let query = build_insert_service_query(&config);
//     let expected_query = "INSERT INTO system.service(id, name, version, online, description, health_check_uri,\n            base_uri, dependencies, exposure,\n            endpoint_name, endpoint_version, endpoint_base_uri, endpoint_port, endpoint_protocol,\n            metric_uri, metric_host, metric_port)\n             VALUES(1, 'smdbv1', 1, false, 'SMDB Service Management Database', 'smdbv1-service.default.svc.cluster.local:7070/health', 'smdbv1-service.default.svc.cluster.local', '{3}', 1,\n                'service-registry', 1, '/', 7070, 1,\n                'metrics', '0.0.0.0', 8080\n            )\n            RETURNING id".to_string();
//     assert_eq!(query, expected_query);
// }
