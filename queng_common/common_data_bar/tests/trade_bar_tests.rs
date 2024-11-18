use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use common_data_bar::TradeBar;

fn get_trade_bar(date_time: DateTime<Utc>) -> TradeBar {
    let symbol_id = 1;
    // let pi = Decimal::new(3141, 3);
    // assert_eq!(pi.to_string(), "3.141");
    let price = Decimal::new(10000, 2);
    let volume = Decimal::new(10000, 2);

    TradeBar::new(symbol_id, date_time, price, volume)
}

#[test]
fn test_date_time() {
    let now = Utc::now();
    let trade_bar = get_trade_bar(now);
    let result = trade_bar.date_time();

    assert_eq!(result, now);
}

#[test]
fn test_price() {
    // Arrange
    let now = Utc::now();
    let trade_bar = get_trade_bar(now);

    // Act
    let result = trade_bar.price();

    // Assert
    let price = Decimal::new(10000, 2);
    assert_eq!(result, price);
}

#[test]
fn test_volume() {
    // Arrange
    let now = Utc::now();
    let trade_bar = get_trade_bar(now);

    // Act
    let result = trade_bar.volume();

    // Assert
    let volume = Decimal::new(10000, 2);
    assert_eq!(result, volume);
}
