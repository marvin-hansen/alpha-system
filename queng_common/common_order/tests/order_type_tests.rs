use common_order::OrderType;

#[test]
fn test_order_type_conversion() {
    assert_eq!(u8::from(OrderType::Limit), 0x1_u8);
    assert_eq!(u8::from(OrderType::Market), 0x2_u8);

    assert_eq!(OrderType::from(0x1_u8), OrderType::Limit);
    assert_eq!(OrderType::from(0x2_u8), OrderType::Market);
    // Test default fallback
    assert_eq!(OrderType::from(0x3_u8), OrderType::Limit);
}

#[test]
fn test_order_type_display() {
    assert_eq!(OrderType::Limit.to_string(), "Limit");
    assert_eq!(OrderType::Market.to_string(), "Market");
}

#[test]
fn test_order_type_debug() {
    assert_eq!(format!("{:?}", OrderType::Limit), "Limit");
    assert_eq!(format!("{:?}", OrderType::Market), "Market");
}

#[test]
fn test_order_type_default() {
    assert_eq!(OrderType::default(), OrderType::Limit);
}

#[test]
fn test_order_type_clone_and_eq() {
    let limit = OrderType::Limit;
    let limit_clone = limit.clone();
    assert_eq!(limit, limit_clone);

    let market = OrderType::Market;
    let market_clone = market.clone();
    assert_eq!(market, market_clone);

    assert_ne!(limit, market);
}
