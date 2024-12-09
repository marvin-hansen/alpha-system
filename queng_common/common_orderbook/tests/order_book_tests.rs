use chrono::Utc;
use common_orderbook::{Ask, Bid, Orderbook};

#[test]
fn test_orderbook_new() {
    let symbol = "AAPL".to_string();
    let time_exchange = Utc::now();
    let time_integration = Utc::now();
    let asks = vec![Ask::default()];
    let bids = vec![Bid::default()];

    let orderbook = Orderbook::new(symbol, time_exchange, time_integration, asks, bids);

    assert_eq!(orderbook.symbol(), "AAPL");
    assert_eq!(orderbook.time_exchange(), time_exchange);
    assert_eq!(orderbook.time_integration(), time_integration);
    assert_eq!(orderbook.asks().len(), 1);
    assert_eq!(orderbook.bids().len(), 1);
}

#[test]
fn test_orderbook_asks_first_n() {
    let symbol = "AAPL".to_string();
    let time_exchange = Utc::now();
    let time_integration = Utc::now();
    let asks = vec![Ask::default(), Ask::default(), Ask::default()];
    let bids = vec![Bid::default()];

    let orderbook = Orderbook::new(symbol, time_exchange, time_integration, asks, bids);

    let first_two_asks = orderbook.asks_first_n(2);
    assert_eq!(first_two_asks.len(), 2);

    let all_asks = orderbook.asks_first_n(100);
    assert_eq!(all_asks.len(), 3);
}

#[test]
fn test_orderbook_bids_first_n() {
    let symbol = "AAPL".to_string();
    let time_exchange = Utc::now();
    let time_integration = Utc::now();
    let asks = vec![Ask::default()];
    let bids = vec![Bid::default(), Bid::default(), Bid::default()];

    let orderbook = Orderbook::new(symbol, time_exchange, time_integration, asks, bids);

    let first_two_bids = orderbook.bids_first_n(2);
    assert_eq!(first_two_bids.len(), 2);

    let all_bids = orderbook.bids_first_n(100);
    assert_eq!(all_bids.len(), 3);
}

#[test]
fn test_orderbook_asks() {
    let symbol = "AAPL".to_string();
    let time_exchange = Utc::now();
    let time_integration = Utc::now();
    let asks = vec![Ask::default(), Ask::default()];
    let bids = vec![Bid::default()];

    let orderbook = Orderbook::new(symbol, time_exchange, time_integration, asks, bids);

    let all_asks = orderbook.asks();
    assert_eq!(all_asks.len(), 2);
}

#[test]
fn test_orderbook_bids() {
    let symbol = "AAPL".to_string();
    let time_exchange = Utc::now();
    let time_integration = Utc::now();
    let asks = vec![Ask::default()];
    let bids = vec![Bid::default(), Bid::default()];

    let orderbook = Orderbook::new(symbol, time_exchange, time_integration, asks, bids);

    let all_bids = orderbook.bids();
    assert_eq!(all_bids.len(), 2);
}

#[test]
fn test_orderbook_symbol() {
    let symbol = "AAPL".to_string();
    let time_exchange = Utc::now();
    let time_integration = Utc::now();
    let asks = vec![Ask::default()];
    let bids = vec![Bid::default()];

    let orderbook = Orderbook::new(symbol, time_exchange, time_integration, asks, bids);

    assert_eq!(orderbook.symbol(), "AAPL");
}

#[test]
fn test_orderbook_time_exchange() {
    let symbol = "AAPL".to_string();
    let time_exchange = Utc::now();
    let time_integration = Utc::now();
    let asks = vec![Ask::default()];
    let bids = vec![Bid::default()];

    let orderbook = Orderbook::new(symbol, time_exchange, time_integration, asks, bids);

    assert_eq!(orderbook.time_exchange(), time_exchange);
}

#[test]
fn test_orderbook_time_integration() {
    let symbol = "AAPL".to_string();
    let time_exchange = Utc::now();
    let time_integration = Utc::now();
    let asks = vec![Ask::default()];
    let bids = vec![Bid::default()];

    let orderbook = Orderbook::new(symbol, time_exchange, time_integration, asks, bids);

    assert_eq!(orderbook.time_integration(), time_integration);
}

#[test]
fn test_orderbook_display() {
    let symbol = "AAPL".to_string();
    let time_exchange = Utc::now();
    let time_integration = Utc::now();
    let asks = vec![Ask::default()];
    let bids = vec![Bid::default()];

    let orderbook = Orderbook::new(symbol, time_exchange, time_integration, asks, bids);

    let expected = format!("{:?}", orderbook);
    let actual = orderbook.to_string();
    assert_eq!(expected, actual);
}

#[test]
fn test_orderbook_clone() {
    let symbol = "AAPL".to_string();
    let time_exchange = Utc::now();
    let time_integration = Utc::now();
    let asks = vec![Ask::default()];
    let bids = vec![Bid::default()];

    let orderbook = Orderbook::new(symbol, time_exchange, time_integration, asks, bids);
    let cloned_orderbook = orderbook.clone();

    assert_eq!(cloned_orderbook.symbol(), "AAPL");
    assert_eq!(cloned_orderbook.time_exchange(), time_exchange);
    assert_eq!(cloned_orderbook.time_integration(), time_integration);
    assert_eq!(cloned_orderbook.asks().len(), 1);
    assert_eq!(cloned_orderbook.bids().len(), 1);
}

#[test]
fn test_orderbook_eq() {
    let symbol = "AAPL".to_string();
    let time_exchange = Utc::now();
    let time_integration = Utc::now();
    let asks = vec![Ask::default()];
    let bids = vec![Bid::default()];

    let orderbook1 = Orderbook::new(
        symbol.clone(),
        time_exchange,
        time_integration,
        asks.clone(),
        bids.clone(),
    );
    let orderbook2 = Orderbook::new(symbol.clone(), time_exchange, time_integration, asks, bids);

    assert_eq!(orderbook1, orderbook2);
}

#[test]
fn test_orderbook_ne() {
    let symbol = "AAPL".to_string();
    let time_exchange = Utc::now();
    let time_integration = Utc::now();
    let asks = vec![Ask::default()];
    let bids = vec![Bid::default()];

    let orderbook1 = Orderbook::new(
        symbol,
        time_exchange,
        time_integration,
        asks.clone(),
        bids.clone(),
    );
    let orderbook2 = Orderbook::new(
        "GOOG".to_string(),
        time_exchange,
        time_integration,
        asks,
        bids,
    );

    assert_ne!(orderbook1, orderbook2);
}
