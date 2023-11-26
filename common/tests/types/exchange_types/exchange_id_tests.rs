use common::prelude::ExchangeID;

#[test]
fn test_unknown_exchange() {
    let exchange_id = ExchangeID::UnknownExchange;
    assert_eq!(format!("{}", exchange_id), "UnknownExchange");
}

#[test]
fn test_binance() {
    let exchange_id = ExchangeID::BNB;
    assert_eq!(format!("{}", exchange_id), "BNB: Binance Spot Exchange");
}

#[test]
fn test_vex() {
    let exchange_id = ExchangeID::VEX;
    assert_eq!(format!("{}", exchange_id), "VEX: Virtual Exchange");
}
