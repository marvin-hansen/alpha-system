use common::prelude::TradeDirection;

#[test]
fn test_display_buy() {
    let direction = TradeDirection::Buy;
    assert_eq!(format!("{:?}", direction), "Buy");
}

#[test]
fn test_display_sell() {
    let direction = TradeDirection::Sell;
    assert_eq!(format!("{:?}", direction), "Sell");
}

#[test]
fn test_display_hold() {
    let direction = TradeDirection::Hold;
    assert_eq!(format!("{:?}", direction), "Hold");
}