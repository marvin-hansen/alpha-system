use common::prelude::DataBar;
use rust_decimal::Decimal;

#[test]
fn test_data_bar_new() {
    let bar = DataBar::new(
        "AAPL".to_string(),
        "1Min".to_string(),
        "2022-01-01".to_string(),
        Decimal::from(100),
        Decimal::from(110),
        Decimal::from(90),
        Decimal::from(105),
        Decimal::from(1000),
    );

    assert_eq!(bar.symbol(), "AAPL");
    assert_eq!(bar.timeframe(), "1Min");
    assert_eq!(bar.period(), "2022-01-01");
    assert_eq!(bar.open(), Decimal::from(100));
    assert_eq!(bar.high(), Decimal::from(110));
    assert_eq!(bar.low(), Decimal::from(90));
    assert_eq!(bar.close(), Decimal::from(105));
    assert_eq!(bar.volume(), Decimal::from(1000));
}

#[test]
fn test_data_bar_display() {
    let bar = DataBar::new(
        "AAPL".to_string(),
        "1Min".to_string(),
        "2022-01-01".to_string(),
        Decimal::from(100),
        Decimal::from(110),
        Decimal::from(90),
        Decimal::from(105),
        Decimal::from(1000),
    );

    assert_eq!(
        format!("{}", bar),
        "DataBar [symbol: AAPL, timeframe: 1Min, period: 2022-01-01, open: 100, high: 110, low: 90, close: 105, volume: 1000]"
    );
}
