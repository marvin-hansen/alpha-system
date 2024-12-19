use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(u8)]
pub enum ExchangeDataIntegrationID {
    #[default]
    NullVal = 0,
    BinanceSpotData = 1,
    BinanceUsdFuturesData = 2,
    BinanceCoinFuturesData = 3,
    BinanceSpotTestnetData = 4,
    BinanceUsdFuturesTestnetData = 5,
    BinanceCoinFuturesTestnetData = 6,
}

impl From<u8> for ExchangeDataIntegrationID {
    fn from(value: u8) -> Self {
        match value {
            0 => ExchangeDataIntegrationID::NullVal,
            1 => ExchangeDataIntegrationID::BinanceSpotData,
            2 => ExchangeDataIntegrationID::BinanceUsdFuturesData,
            3 => ExchangeDataIntegrationID::BinanceCoinFuturesData,
            4 => ExchangeDataIntegrationID::BinanceSpotTestnetData,
            5 => ExchangeDataIntegrationID::BinanceUsdFuturesTestnetData,
            6 => ExchangeDataIntegrationID::BinanceCoinFuturesTestnetData,
            _ => panic!("Invalid exchange data integration"),
        }
    }
}

impl From<ExchangeDataIntegrationID> for u8 {
    fn from(val: ExchangeDataIntegrationID) -> Self {
        val as u8
    }
}

impl From<u16> for ExchangeDataIntegrationID {
    fn from(value: u16) -> Self {
        match value {
            0 => ExchangeDataIntegrationID::NullVal,
            1 => ExchangeDataIntegrationID::BinanceSpotData,
            2 => ExchangeDataIntegrationID::BinanceUsdFuturesData,
            3 => ExchangeDataIntegrationID::BinanceCoinFuturesData,
            4 => ExchangeDataIntegrationID::BinanceSpotTestnetData,
            5 => ExchangeDataIntegrationID::BinanceUsdFuturesTestnetData,
            6 => ExchangeDataIntegrationID::BinanceCoinFuturesTestnetData,
            _ => panic!("Invalid exchange data integration"),
        }
    }
}

impl From<ExchangeDataIntegrationID> for u16 {
    fn from(val: ExchangeDataIntegrationID) -> Self {
        val as u16
    }
}

impl Display for ExchangeDataIntegrationID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
