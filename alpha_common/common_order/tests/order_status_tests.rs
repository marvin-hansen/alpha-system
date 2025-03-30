/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_order::OrderStatus;

#[test]
fn test_order_status_conversion() {
    // Test all conversions from enum to u8
    assert_eq!(u8::from(OrderStatus::ROUTING), 1);
    assert_eq!(u8::from(OrderStatus::ROUTED), 2);
    assert_eq!(u8::from(OrderStatus::RECEIVED), 3);
    assert_eq!(u8::from(OrderStatus::PendingNew), 4);
    assert_eq!(u8::from(OrderStatus::NEW), 5);
    assert_eq!(u8::from(OrderStatus::PartiallyFilled), 6);
    assert_eq!(u8::from(OrderStatus::Filled), 7);
    assert_eq!(u8::from(OrderStatus::PendingCancel), 8);
    assert_eq!(u8::from(OrderStatus::Canceled), 9);
    assert_eq!(u8::from(OrderStatus::PendingReplace), 10);
    assert_eq!(u8::from(OrderStatus::Replaced), 11);
    assert_eq!(u8::from(OrderStatus::Rejected), 12);

    // Test all conversions from u8 to enum
    assert_eq!(OrderStatus::from(1), OrderStatus::ROUTING);
    assert_eq!(OrderStatus::from(2), OrderStatus::ROUTED);
    assert_eq!(OrderStatus::from(3), OrderStatus::RECEIVED);
    assert_eq!(OrderStatus::from(4), OrderStatus::PendingNew);
    assert_eq!(OrderStatus::from(5), OrderStatus::NEW);
    assert_eq!(OrderStatus::from(6), OrderStatus::PartiallyFilled);
    assert_eq!(OrderStatus::from(7), OrderStatus::Filled);
    assert_eq!(OrderStatus::from(8), OrderStatus::PendingCancel);
    assert_eq!(OrderStatus::from(9), OrderStatus::Canceled);
    assert_eq!(OrderStatus::from(10), OrderStatus::PendingReplace);
    assert_eq!(OrderStatus::from(11), OrderStatus::Replaced);
    assert_eq!(OrderStatus::from(12), OrderStatus::Rejected);

    // Test default fallback for invalid value
    assert_eq!(OrderStatus::from(13), OrderStatus::ROUTING);
}

#[test]
fn test_order_status_display() {
    assert_eq!(OrderStatus::ROUTING.to_string(), "ROUTING");
    assert_eq!(OrderStatus::ROUTED.to_string(), "ROUTED");
    assert_eq!(OrderStatus::RECEIVED.to_string(), "RECEIVED");
    assert_eq!(OrderStatus::PendingNew.to_string(), "PendingNew");
    assert_eq!(OrderStatus::NEW.to_string(), "NEW");
    assert_eq!(OrderStatus::PartiallyFilled.to_string(), "PartiallyFilled");
    assert_eq!(OrderStatus::Filled.to_string(), "Filled");
    assert_eq!(OrderStatus::PendingCancel.to_string(), "PendingCancel");
    assert_eq!(OrderStatus::Canceled.to_string(), "Canceled");
    assert_eq!(OrderStatus::PendingReplace.to_string(), "PendingReplace");
    assert_eq!(OrderStatus::Replaced.to_string(), "Replaced");
    assert_eq!(OrderStatus::Rejected.to_string(), "Rejected");
}

#[test]
fn test_order_status_debug() {
    assert_eq!(format!("{:?}", OrderStatus::ROUTING), "ROUTING");
    assert_eq!(format!("{:?}", OrderStatus::ROUTED), "ROUTED");
    assert_eq!(format!("{:?}", OrderStatus::RECEIVED), "RECEIVED");
    assert_eq!(format!("{:?}", OrderStatus::PendingNew), "PendingNew");
    assert_eq!(format!("{:?}", OrderStatus::NEW), "NEW");
    assert_eq!(
        format!("{:?}", OrderStatus::PartiallyFilled),
        "PartiallyFilled"
    );
    assert_eq!(format!("{:?}", OrderStatus::Filled), "Filled");
    assert_eq!(format!("{:?}", OrderStatus::PendingCancel), "PendingCancel");
    assert_eq!(format!("{:?}", OrderStatus::Canceled), "Canceled");
    assert_eq!(
        format!("{:?}", OrderStatus::PendingReplace),
        "PendingReplace"
    );
    assert_eq!(format!("{:?}", OrderStatus::Replaced), "Replaced");
    assert_eq!(format!("{:?}", OrderStatus::Rejected), "Rejected");
}

#[test]
fn test_order_status_clone_and_eq() {
    let routing = OrderStatus::ROUTING;
    let routing_clone = routing.clone();
    assert_eq!(routing, routing_clone);

    let routed = OrderStatus::ROUTED;
    assert_ne!(routing, routed);

    let received = OrderStatus::RECEIVED;
    assert_ne!(routing, received);
    assert_ne!(routed, received);

    // Test a few more combinations
    assert_ne!(OrderStatus::NEW, OrderStatus::Filled);
    assert_ne!(OrderStatus::PartiallyFilled, OrderStatus::Canceled);
    assert_ne!(OrderStatus::PendingReplace, OrderStatus::Rejected);
}
