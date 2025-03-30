/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_config::{ProtocolType, ServiceID};
use dbgw_specs::dbgw_service_config;

#[test]
fn test_dbgw_service_config() {
    let service_config = dbgw_service_config();

    assert_eq!(service_config.svc_id(), &ServiceID::DBGW);
    assert_eq!(service_config.name(), "dbgw");
    assert_eq!(service_config.version(), 1);
    assert!(!service_config.online());
    assert_eq!(service_config.description(), "DBGW gives access to the DB");
    assert_eq!(
        service_config.health_check_uri(),
        "dbgw-service.default.svc.cluster.local:9090/health"
    );
    assert_eq!(
        service_config.cluster_uri(),
        "dbgw-service.default.svc.cluster.local"
    );
    assert_eq!(service_config.dependencies().len(), 0);
    let endpoint = service_config.service_endpoint();
    assert_eq!(endpoint.name(), "dbgw Endpoint");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.uri(), String::from("/"));
    assert_eq!(endpoint.port(), 9090);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
}
