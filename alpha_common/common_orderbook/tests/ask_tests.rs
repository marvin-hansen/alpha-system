/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_orderbook::Ask;
use rust_decimal::Decimal;

#[test]
fn test_new() {
    let price = Decimal::new(105, 1);
    let quantity = Decimal::new(20, 1);
    let ask_bid = Ask::new(price, quantity);
    assert_eq!(ask_bid.price(), price);
    assert_eq!(ask_bid.quantity(), quantity);
}

#[test]
fn test_price() {
    let price = Decimal::new(105, 1);
    let quantity = Decimal::new(20, 1);
    let ask_bid = Ask::new(price, quantity);
    assert_eq!(ask_bid.price(), price);
}

#[test]
fn test_quantity() {
    let price = Decimal::new(105, 1);
    let quantity = Decimal::new(20, 1);
    let ask_bid = Ask::new(price, quantity);
    assert_eq!(ask_bid.quantity(), quantity);
}

#[test]
fn test_display() {
    let price = Decimal::new(105, 1);
    let quantity = Decimal::new(20, 1);
    let ask_bid = Ask::new(price, quantity);
    assert_eq!(
        format!("{:?}", ask_bid),
        "Ask { price: 10.5, quantity: 2.0 }"
    );
}

#[test]
fn test_debug() {
    let price = Decimal::new(105, 1);
    let quantity = Decimal::new(20, 1);
    let ask_bid = Ask::new(price, quantity);
    assert_eq!(
        format!("{:?}", ask_bid),
        "Ask { price: 10.5, quantity: 2.0 }"
    );
}

#[test]
fn test_default() {
    let ask_bid = Ask::default();
    assert_eq!(ask_bid.price(), Decimal::new(0, 0));
    assert_eq!(ask_bid.quantity(), Decimal::new(0, 0));
}

#[test]
fn test_eq() {
    let price = Decimal::new(105, 1);
    let quantity = Decimal::new(20, 1);
    let ask_bid1 = Ask::new(price, quantity);
    let ask_bid2 = Ask::new(price, quantity);
    assert_eq!(ask_bid1, ask_bid2);
}

#[test]
fn test_neq_price() {
    let price1 = Decimal::new(105, 1);
    let price2 = Decimal::new(106, 1);
    let quantity = Decimal::new(20, 1);
    let ask_bid1 = Ask::new(price1, quantity);
    let ask_bid2 = Ask::new(price2, quantity);
    assert_ne!(ask_bid1, ask_bid2);
}

#[test]
fn test_neq_quantity() {
    let price = Decimal::new(105, 1);
    let quantity1 = Decimal::new(20, 1);
    let quantity2 = Decimal::new(21, 1);
    let ask_bid1 = Ask::new(price, quantity1);
    let ask_bid2 = Ask::new(price, quantity2);
    assert_ne!(ask_bid1, ask_bid2);
}

#[test]
fn test_clone() {
    let price = Decimal::new(105, 1);
    let quantity = Decimal::new(20, 1);
    let ask_bid1 = Ask::new(price, quantity);
    let ask_bid2 = ask_bid1.clone();
    assert_eq!(ask_bid1, ask_bid2);
}
