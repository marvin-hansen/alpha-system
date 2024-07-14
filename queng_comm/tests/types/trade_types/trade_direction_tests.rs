use common::prelude::TradeDirection;

#[test]
fn test_debug() {
    let direction = TradeDirection::Buy;
    assert_eq!(format!("{:?}", direction), "Buy");

    let direction = TradeDirection::Sell;
    assert_eq!(format!("{:?}", direction), "Sell");

    let direction = TradeDirection::Hold;
    assert_eq!(format!("{:?}", direction), "Hold");
}

#[test]
fn test_display() {
    let direction = TradeDirection::Buy;
    assert_eq!(format!("{}", direction), "Buy");

    let direction = TradeDirection::Sell;
    assert_eq!(format!("{}", direction), "Sell");

    let direction = TradeDirection::Hold;
    assert_eq!(format!("{}", direction), "Hold");
}
