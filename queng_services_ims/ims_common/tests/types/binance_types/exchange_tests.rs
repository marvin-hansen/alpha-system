use ims_common::prelude::{BinanceExchangeInfo, BinanceSymbol};

#[test]
fn test_exchange_info_struct() {
    let symbol = BinanceSymbol {
        symbol: "BTCUSDT".to_string(),
        status: "TRADING".to_string(),
    };

    let exchange_info = BinanceExchangeInfo {
        timezone: "UTC".to_string(),
        server_time: 9000,
        symbols: vec![symbol.clone()],
    };

    assert_eq!(exchange_info.timezone, "UTC");
    assert_eq!(exchange_info.server_time, 9000);
    assert_eq!(exchange_info.symbols, vec![symbol.clone()]);
}
