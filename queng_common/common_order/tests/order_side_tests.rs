use common_order::OrderSide;

#[test]
fn test_order_side_conversion() {
    assert_eq!(u8::from(OrderSide::Buy), 0x1_u8);
    assert_eq!(u8::from(OrderSide::Sell), 0x2_u8);

    assert_eq!(OrderSide::from(0x1_u8), OrderSide::Buy);
    assert_eq!(OrderSide::from(0x2_u8), OrderSide::Sell);
    // Test default fallback
    assert_eq!(OrderSide::from(0x3_u8), OrderSide::Buy);
}

#[test]
fn test_order_side_display() {
    assert_eq!(OrderSide::Buy.to_string(), "Buy");
    assert_eq!(OrderSide::Sell.to_string(), "Sell");
}

#[test]
fn test_order_side_debug() {
    assert_eq!(format!("{:?}", OrderSide::Buy), "Buy");
    assert_eq!(format!("{:?}", OrderSide::Sell), "Sell");
}

#[test]
fn test_order_side_clone_and_eq() {
    let buy = OrderSide::Buy;
    let buy_clone = buy.clone();
    assert_eq!(buy, buy_clone);

    let sell = OrderSide::Sell;
    let sell_clone = sell.clone();
    assert_eq!(sell, sell_clone);

    assert_ne!(buy, sell);
}
