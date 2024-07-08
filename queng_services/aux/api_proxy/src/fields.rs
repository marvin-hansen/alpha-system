pub(crate) const BASE_URL: &str = "https://reference-data-api.kaiko.io/v1/";
pub(crate) const CEX_URL: &str = "https://docs.kaiko.com/#cex-coverage";

// https://docs.kaiko.com/#dex-coverage
pub(crate) const DEX: [&str; 12] = [
    "usp2", "usp3", "sush", "pksp", "curv", "blcr", "blc2", "tjv1", "tjv2", "tjv3", "qsp2", "qsp3",
];

pub(crate) const INACTIVE_EXCHANGES: [&str; 35] = [
    "ACX",
    "Allcoin",
    "AnyBits",
    "BTC-e",
    "BTC38",
    "Binance V2",
    "BinanceJEX",
    "BitForex",
    "BitMarket",
    "Bitibu",
    "Bitlish",
    "Bittrex", // Gone, filled for liquidation
    "C-CEX",
    "CRCO",
    "Cobinhood",
    "CoinEgg",
    "Coinflex", // Gone. No website.
    "Coinfloor",
    "Crosstower",
    "EXX",
    "Ethfinex",
    "Etorox",
    "FTX US",
    "FTX",
    "Gatecoin",
    "LGOMarkets",
    "LocalBitcoins",
    "MtGox",
    "NegocieCoins",
    "OSL",
    "Quoine",
    "Stronghold",
    "TheRockTrading",
    "UEX",
    "bx.in.th",
];
