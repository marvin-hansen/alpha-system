use common::prelude::DataBar;
use rust_decimal::Decimal;

#[test]
fn test_data_bar_new() {
    let bar = DataBar::new(
        "2020-01-01T00:00:00".to_string(),
        "AAPL".to_string(),
        Decimal::from(100),
        Decimal::from(110),
        Decimal::from(90),
        Decimal::from(105),
        Decimal::from(1000),
        Decimal::from(500),
    );

    assert_eq!(bar.date_time(), "2020-01-01T00:00:00");
    assert_eq!(bar.symbol(), "AAPL");
    assert_eq!(bar.open(), Decimal::from(100));
    assert_eq!(bar.high(), Decimal::from(110));
    assert_eq!(bar.low(), Decimal::from(90));
    assert_eq!(bar.close(), Decimal::from(105));
    assert_eq!(bar.volume(), Decimal::from(1000));
    assert_eq!(bar.trades(), Decimal::from(500));
}

#[test]
fn test_data_bar_display() {
    let actual = DataBar::new(
        "2020-01-01T00:00:00".to_string(),
        "AAPL".to_string(),
        Decimal::from(100),
        Decimal::from(110),
        Decimal::from(90),
        Decimal::from(105),
        Decimal::from(1000),
        Decimal::from(500),
    );

    let expected = "DataTime: 2020-01-01T00:00:00,\n Symbol AAPL,\n Open 100,\n High 110,\n Low 90,\n Close 105,\n Volume 1000,\n Trades 500";

    assert_eq!(actual.to_string(), expected);
}
