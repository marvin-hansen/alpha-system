use common::prelude::TradeStrategyType;

#[test]
fn test_display_default() {
    let expected = "BuyHold";
    let actual = TradeStrategyType::default().to_string();
    assert_eq!(expected, actual);
}

#[test]
fn test_display_buy_hold() {
    let expected = "BuyHold";
    let actual = TradeStrategyType::BuyHold.to_string();
    assert_eq!(expected, actual);
}

#[test]
fn test_display_turbo_trend() {
    let expected = "TurboTrend";
    let actual = TradeStrategyType::TurboTrend.to_string();
    assert_eq!(expected, actual);
}

#[test]
fn test_display_causal_breakout() {
    let expected = "CausalBreakout";
    let actual = TradeStrategyType::CausalBreakout.to_string();
    assert_eq!(expected, actual);
}
