/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use chrono::Utc;
use common_order::OrderFill;
use rust_decimal::Decimal;

#[test]
fn test_order_fill_new() {
    let time = Utc::now();
    let timestamp = 1639123200;
    let price = Decimal::new(10050, 2); // 100.50
    let quantity = Decimal::new(105, 1); // 10.5

    let fill = OrderFill::new(time, timestamp, price, quantity);

    assert_eq!(fill.time(), time);
    assert_eq!(fill.timestamp(), timestamp);
    assert_eq!(fill.price(), price);
    assert_eq!(fill.quantity(), quantity);
}

#[test]
fn test_order_fill_display() {
    let time = Utc::now();
    let timestamp = 1639123200;
    let price = Decimal::new(10050, 2); // 100.50
    let quantity = Decimal::new(105, 1); // 10.5

    let fill = OrderFill::new(time, timestamp, price, quantity);
    let display_string = fill.to_string();

    // Verify the display string contains all the important information
    assert!(display_string.contains(&price.to_string()));
    assert!(display_string.contains(&quantity.to_string()));
    assert!(display_string.contains(&timestamp.to_string()));
}

#[test]
fn test_order_fill_debug() {
    let time = Utc::now();
    let timestamp = 1639123200;
    let price = Decimal::new(10050, 2); // 100.50
    let quantity = Decimal::new(105, 1); // 10.5

    let fill = OrderFill::new(time, timestamp, price, quantity);
    let debug_string = format!("{:?}", fill);

    // Verify the debug output contains all fields
    assert!(debug_string.contains(&price.to_string()));
    assert!(debug_string.contains(&quantity.to_string()));
    assert!(debug_string.contains(&timestamp.to_string()));
}

#[test]
fn test_order_fill_clone_and_eq() {
    let time = Utc::now();
    let timestamp = 1639123200;
    let price = Decimal::new(10050, 2); // 100.50
    let quantity = Decimal::new(105, 1); // 10.5

    let fill = OrderFill::new(time, timestamp, price, quantity);
    let fill_clone = fill.clone();

    assert_eq!(fill, fill_clone);

    // Create a different fill with different values
    let different_fill = OrderFill::new(
        time,
        timestamp + 1,
        Decimal::new(10150, 2), // 101.50
        Decimal::new(115, 1),   // 11.5
    );

    assert_ne!(fill, different_fill);
}

#[test]
fn test_order_fill_with_zero_values() {
    let time = Utc::now();
    let timestamp = 0;
    let price = Decimal::ZERO;
    let quantity = Decimal::ZERO;

    let fill = OrderFill::new(time, timestamp, price, quantity);

    assert_eq!(fill.price(), Decimal::ZERO);
    assert_eq!(fill.quantity(), Decimal::ZERO);
    assert_eq!(fill.timestamp(), 0);
}

#[test]
fn test_order_fill_with_large_values() {
    let time = Utc::now();
    let timestamp = u64::MAX;
    let price = Decimal::new(99999999999, 5); // 999999.99999
    let quantity = Decimal::new(99999999999, 5); // 999999.99999

    let fill = OrderFill::new(time, timestamp, price, quantity);

    assert_eq!(fill.price(), price);
    assert_eq!(fill.quantity(), quantity);
    assert_eq!(fill.timestamp(), timestamp);
}
