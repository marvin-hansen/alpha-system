use common::prelude::ServiceID;

#[test]
fn test_debug() {
    let e2 = ServiceID::MEMGRAPH;
    assert_eq!(format!("{:?}", e2), "MEMGRAPH");

    let e3 = ServiceID::SMDB;
    assert_eq!(format!("{:?}", e3), "SMDB");

    let e4 = ServiceID::CMDB;
    assert_eq!(format!("{:?}", e4), "CMDB");
}

#[test]
fn test_display() {
    let e2 = ServiceID::MEMGRAPH;
    assert_eq!(format!("{}", e2), "MEMGRAPH");

    let e3 = ServiceID::SMDB;
    assert_eq!(format!("{}", e3), "SMDB");

    let e4 = ServiceID::CMDB;
    assert_eq!(format!("{}", e4), "CMDB");
}
