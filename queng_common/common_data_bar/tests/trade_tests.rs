use chrono::{DateTime, TimeZone, Utc};
use common_data_bar::trade::Trade;
use rust_decimal::Decimal;

#[test]
fn test_trade_new() {
    let symbol = "AAPL".to_string();
    let date_time = Utc.with_ymd_and_hms(2024, 12, 8, 10, 43, 32).unwrap();
    let price = Decimal::new(15000, 2); // 150.00
    let quantity = Decimal::new(100, 0); // 100

    let trade = Trade::new(symbol.clone(), date_time, price, quantity);

    assert_eq!(trade.symbol(), "AAPL");
    assert_eq!(trade.date_time(), date_time);
    assert_eq!(trade.price(), price);
    assert_eq!(trade.quantity(), quantity);
}

#[test]
fn test_trade_default() {
    let trade = Trade::default();

    assert_eq!(trade.symbol(), "");
    assert_eq!(trade.price(), Decimal::new(0, 0));
    assert_eq!(trade.quantity(), Decimal::new(0, 0));
}

#[test]
fn test_trade_clone_and_eq() {
    let symbol = "TSLA".to_string();
    let date_time = Utc.with_ymd_and_hms(2024, 12, 8, 10, 43, 32).unwrap();
    let price = Decimal::new(25000, 2); // 250.00
    let quantity = Decimal::new(50, 0); // 50

    let trade1 = Trade::new(symbol.clone(), date_time, price, quantity);
    let trade2 = trade1.clone();

    assert_eq!(trade1, trade2);
}

#[test]
fn test_trade_display() {
    let symbol = "GOOGL".to_string();
    let date_time = Utc.with_ymd_and_hms(2024, 12, 8, 10, 43, 32).unwrap();
    let price = Decimal::new(135000, 2); // 1350.00
    let quantity = Decimal::new(10, 0); // 10

    let trade = Trade::new(symbol, date_time, price, quantity);
    let display_string = format!("{}", trade);

    assert!(display_string.contains("GOOGL"));
    assert!(display_string.contains("1350"));
    assert!(display_string.contains("10"));
    assert!(display_string.contains(&date_time.to_string()));
}
