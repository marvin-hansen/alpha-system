// use common_config::prelude::ServiceConfig;
// use smdb_specs::smdb_service_config;
//
// fn get_service_config() -> ServiceConfig {
//     smdb_service_config()
// }

// #[test]
// fn test_build_update_service_query() {
//     let config = get_service_config();
//     let actual_query = build_update_service_query(&config);
//     let expected_query = "UPDATE\n                system.service\n            SET\n                name='smdbv1',\n                version=1,\n                online=false,\n                description='SMDB Service Management Database',\n                health_check_uri='smdbv1-service.default.svc.cluster.local:7070/health',\n                base_uri='smdbv1-service.default.svc.cluster.local',\n                dependencies='{3}',\n                exposure=1,\n                endpoint_name='service-registry',\n                endpoint_version=1,\n                endpoint_base_uri='/',\n                endpoint_port=7070,\n                endpoint_protocol=1,\n                metric_uri='metrics',\n                metric_host='0.0.0.0',\n                metric_port=8080\n            WHERE\n                id=1\n            RETURNING service.online".to_string();
//     assert_eq!(actual_query, expected_query);
// }
//
