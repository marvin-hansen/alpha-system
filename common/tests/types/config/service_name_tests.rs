use common::types::config::service_name::ServiceName;

#[test]
fn test_default() {
    let service_name = ServiceName::default();
    assert_eq!(service_name, ServiceName::UNKNOWN);
}

#[test]
fn test_debug() {
    let e1 = ServiceName::UNKNOWN;
    assert_eq!(format!("{:?}", e1), "UNKNOWN");

    let e2 = ServiceName::MEMGRAPH;
    assert_eq!(format!("{:?}", e2), "MEMGRAPH");

    let e3 = ServiceName::SMDB;
    assert_eq!(format!("{:?}", e3), "SMDB");

    let e4 = ServiceName::CMDB;
    assert_eq!(format!("{:?}", e4), "CMDB");
}

#[test]
fn test_display() {
    let e1 = ServiceName::UNKNOWN;
    assert_eq!(format!("{}", e1), "UNKNOWN");

    let e2 = ServiceName::MEMGRAPH;
    assert_eq!(format!("{}", e2), "MEMGRAPH");

    let e3 = ServiceName::SMDB;
    assert_eq!(format!("{}", e3), "SMDB");

    let e4 = ServiceName::CMDB;
    assert_eq!(format!("{}", e4), "CMDB");
}
