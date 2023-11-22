use common::prelude::ServiceID;

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
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", ServiceID::Default), "Default");
    assert_eq!(format!("{}", ServiceID::SMDB), "SMDB");
    assert_eq!(format!("{}", ServiceID::CMDB), "CMDB");
    assert_eq!(format!("{}", ServiceID::DBGW), "DBGW");
    assert_eq!(format!("{}", ServiceID::QDGW), "QDGW");
}
