pub(crate) const BASE_URL: &str = "https://reference-data-api.kaiko.io/v1/";
pub(crate) const LOCAL_PROXY_URL: &str = "http://127.0.0.1:7777/";
pub(crate) const CLUSTER_PROXY_URL: &str =
    "http://apiproxy-service.default.svc.cluster.local:7777/";

pub(crate) const CI_PROXY_URL: &str = "http://127.0.0.1:7777/";

// https://docs.kaiko.com/#dex-coverage
pub(crate) const DEX: [&str; 12] = [
    "usp2", "usp3", "sush", "pksp", "curv", "blcr", "blc2", "tjv1", "tjv2", "tjv3", "qsp2", "qsp3",
];

pub(crate) const NON_UNIQUE_EXCHANGE_INSTRUMENT_ID: [&str; 1] = ["1000NEIROCTOUSDT"];

pub(crate) const ERRATA_INSTRUMENT_ID: [(&str, &str); 7] = [
    ("bbit", "POLUSDT"), // bbit: Instrument_class should be SPOT https://www.bybit.com/en/trade/spot/POL/USDT
    ("bbit", "POLPERP"), // bbit: quote_asset should be  USDC https://www.bybit.com/trade/futures/usdc/POL-PERP
    // bgdm
    ("bgdm", "LUNCUSDT"), // bgdm: base_asset should be LUNAC https://www.bitget.com/futures/usdt/LUNCUSDT
    ("bgdm", "BTCPERP"), // bgdm: quote_asset should be  USDC https://www.bitget.com/futures/usdc/BTCPERP
    ("bgdm", "ETHPERP"), // bgdm: quote_asset should be  USDC https://www.bitget.com/futures/usdc/ETHPERP
    // hbdm / HTX https://www.htx.com
    ("hbdm", "peopleusdt"), // hbdm: Instrument_class should be SPOT https://www.htx.com/trade/people_usdt/
    // Okex https://www.okx.com/
    ("okex", "NEIROETH-USDT-SWAP"), //okex: base_asset should be NEIROETH  https://www.okx.com/trade-swap-strategy/neiroeth-usdt-swap
];

pub(crate) const NON_TRADE_INSTRUMENT_ID: [&str; 5] =
    ["etf", "option", "option_combo", "future_combo", "future"];

// https://docs.kaiko.com/coverage/centralized-exchanges
pub(crate) const ACTIVE_EXCHANGES: [&str; 64] = [
    "bcex",
    "btc-alpha",
    "btcc",
    "bequant",
    "bibox",
    "bitbay",
    "bigone",
    "binance",
    "binance futures",
    // "binance options",
    // "binanceus",
    "bit-z",
    "bit2c",
    "bitget dm",
    "bitget spot",
    "bitflyer",
    "bitbank",
    "bitfinex",
    "bithumb",
    "bitmex",
    "bitpanda",
    "bitso",
    "bitstamp",
    "btcturk",
    "btcbox",
    "bullish",
    "bybit",
    "bybit v2",
    "bybit spot",
    "cex.io",
    "cme",
    "coinex",
    "coinbase",
    "coincheck",
    "coinmate",
    "coinone",
    "currency.com",
    "crypto.com",
    "delta exchange",
    "deribit",
    "gate.io",
    "gate.io derivative market",
    "gemini",
    "hitbtc",
    "huobi",
    "huobi derivative market",
    "independent reserve",
    "itbit",
    "korbit",
    "kraken",
    "kucoin",
    "lma",
    "mexc",
    "mercadobitcoin",
    "novadax",
    "okcoin",
    "okex",
    "poloniex",
    "southxchange",
    "tidebit",
    "tidex",
    "upbit",
    "yobit",
    "vaultoro",
    "zaif",
    "zb",
];

// Updated relative to website
// "Bittrex",  Gone, filled for liquidation
// "Coinflex", Gone. No website.
//  "OSL",    Gone. website. blocked
// "cryptofacilities", // Nearly all futures are incorrectly as perpetuals.
// pub(crate) const INACTIVE_EXCHANGES: [&str; 36] = [
//     "ACX",
//     "Allcoin",
//     "AnyBits",
//     "BTC-e",
//     "BTC38",
//     "Binance V2",
//     "BinanceJEX",
//     "BitForex",
//     "BitMarket",
//     "Bitibu",
//     "Bitlish",
//     "Bittrex", // Gone, filled for liquidation
//     "C-CEX",
//     "CRCO",
//     "Cobinhood",
//     "CoinEgg",
//     "Coinflex", // Gone. No website.
//     "Coinfloor",
//     "Crosstower",
//     "EXX",
//     "Ethfinex",
//     "Etorox",
//     "FTX",
//     "FTX US",
//     "Gatecoin",
//     "LGOMarkets",
//     "LocalBitcoins",
//     "MtGox",
//     "NegocieCoins",
//     "OSL",  // Gone. No website.
//     "Quoine",
//     "Stronghold",
//     "The Ocean",
//     "TheRockTrading",
//     "UEX",
//     "bx.in.th",
// ];
