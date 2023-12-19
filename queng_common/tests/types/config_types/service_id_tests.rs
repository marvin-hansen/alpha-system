use common::prelude::ServiceID;

#[test]
fn test_default() {
    let service_name = ServiceID::default();
    assert_eq!(service_name, ServiceID::Default);
}

#[test]
fn test_from_i32() {
    assert_eq!(ServiceID::from(0x0_i32), ServiceID::Default);
    assert_eq!(ServiceID::from(0x1_i32), ServiceID::SMDB);
    assert_eq!(ServiceID::from(0x2_i32), ServiceID::CMDB);
    assert_eq!(ServiceID::from(0x3_i32), ServiceID::DBGW);
    assert_eq!(ServiceID::from(0x4_i32), ServiceID::QDGW);
    assert_eq!(ServiceID::from(0x5_i32), ServiceID::Default);
}

#[test]
fn test_debug() {
    let e = ServiceID::Default;
    assert_eq!(format!("{:?}", e), "Default");

    let e = ServiceID::SMDB;
    assert_eq!(format!("{:?}", e), "SMDB");

    let e = ServiceID::CMDB;
    assert_eq!(format!("{:?}", e), "CMDB");

    let e = ServiceID::DBGW;
    assert_eq!(format!("{:?}", e), "DBGW");

    let e = ServiceID::QDGW;
    assert_eq!(format!("{:?}", e), "QDGW");
}

#[test]
fn test_from_string() {
    assert_eq!(ServiceID::from_string("Default"), Some(ServiceID::Default));
    assert_eq!(ServiceID::from_string("SMDB"), Some(ServiceID::SMDB));
    assert_eq!(ServiceID::from_string("CMDB"), Some(ServiceID::CMDB));
    assert_eq!(ServiceID::from_string("DBGW"), Some(ServiceID::DBGW));
    assert_eq!(ServiceID::from_string("QDGW"), Some(ServiceID::QDGW));
    assert_eq!(ServiceID::from_string("Unknown"), None);
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", ServiceID::Default), "Default");
    assert_eq!(format!("{}", ServiceID::SMDB), "SMDB");
    assert_eq!(format!("{}", ServiceID::CMDB), "CMDB");
    assert_eq!(format!("{}", ServiceID::DBGW), "DBGW");
    assert_eq!(format!("{}", ServiceID::QDGW), "QDGW");
}
