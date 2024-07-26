use ims_common::prelude::BinanceSymbol;

#[test]
fn test_symbol_struct() {
    let symbol = BinanceSymbol {
        symbol: "BTCUSDT".to_string(),
        status: "TRADING".to_string(),
    };

    assert_eq!(symbol.symbol, "BTCUSDT");
    assert_eq!(symbol.status, "TRADING");
}
