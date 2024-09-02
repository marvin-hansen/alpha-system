use common_exchange::prelude::ExchangeID;
use proto_mddb_utils::request_utils_proto::{
    get_exchange_request, get_symbol_id_request, get_symbol_request,
};

#[test]
fn test_get_exchange_request_conversion() {
    let exchange_id = ExchangeID::Kraken;
    let request = get_exchange_request(exchange_id);
    assert_eq!(request.exchange_id, exchange_id as i32);
}

#[test]
fn test_get_exchange_request_nullval() {
    let exchange_id = ExchangeID::NullVal;
    let request = get_exchange_request(exchange_id);
    assert_eq!(request.exchange_id, 0);
}

#[test]
fn test_get_exchange_request_all_variants() {
    let variants = [
        ExchangeID::NullVal,
        ExchangeID::Kraken,
        ExchangeID::COINBASE,
        ExchangeID::VEX,
        ExchangeID::Binance,
    ];

    for &variant in &variants {
        let request = get_exchange_request(variant);
        assert_eq!(request.exchange_id, variant as i32);
    }
}

#[test]
fn test_get_symbol_request_conversion() {
    let exchange_id = ExchangeID::Kraken;
    let symbol_id = 12345;
    let request = get_symbol_request(exchange_id, symbol_id);

    assert_eq!(request.exchange_id, exchange_id as i32);
    assert_eq!(request.symbol_id, symbol_id as i32);
}

#[test]
fn test_get_symbol_request_nullval() {
    let exchange_id = ExchangeID::NullVal;
    let symbol_id = 12345;
    let request = get_symbol_request(exchange_id, symbol_id);

    assert_eq!(request.exchange_id, 0);
    assert_eq!(request.symbol_id, symbol_id as i32);
}

#[test]
fn test_get_symbol_request_max_symbol_id() {
    let exchange_id = ExchangeID::Binance;
    let symbol_id = u16::MAX;
    let request = get_symbol_request(exchange_id, symbol_id);

    assert_eq!(request.exchange_id, exchange_id as i32);
    assert_eq!(request.symbol_id, symbol_id as i32);
}

#[test]
fn test_get_symbol_id_request_conversion() {
    let exchange_id = ExchangeID::Kraken;
    let symbol = "BTCUSD".to_string();
    let request = get_symbol_id_request(exchange_id, symbol.clone());

    assert_eq!(request.exchange_id, exchange_id as i32);
    assert_eq!(request.symbol, symbol);
}

#[test]
fn test_get_symbol_id_request_empty_symbol() {
    let exchange_id = ExchangeID::COINBASE;
    let symbol = "".to_string();
    let request = get_symbol_id_request(exchange_id, symbol.clone());

    assert_eq!(request.exchange_id, exchange_id as i32);
    assert_eq!(request.symbol, symbol);
}

#[test]
fn test_get_symbol_id_request_invalid_exchange_id() {
    let exchange_id = ExchangeID::from(255_u8); // Invalid ExchangeID
    let symbol = "ETHUSD".to_string();
    let request = get_symbol_id_request(exchange_id, symbol.clone());

    assert_eq!(request.exchange_id, ExchangeID::NullVal as i32);
    assert_eq!(request.symbol, symbol);
}
