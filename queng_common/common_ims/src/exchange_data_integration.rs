use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ExchangeDataIntegrationID {
    NullVal = 0,
    BinanceData = 1,
}

impl Display for ExchangeDataIntegrationID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<u8> for ExchangeDataIntegrationID {
    fn from(value: u8) -> Self {
        match value {
            0 => ExchangeDataIntegrationID::NullVal,
            1 => ExchangeDataIntegrationID::BinanceData,
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
            1 => ExchangeDataIntegrationID::BinanceData,
            _ => panic!("Invalid exchange data integration"),
        }
    }
}

impl From<ExchangeDataIntegrationID> for u16 {
    fn from(val: ExchangeDataIntegrationID) -> Self {
        val as u16
    }
}

impl From<u32> for ExchangeDataIntegrationID {
    fn from(value: u32) -> Self {
        match value {
            0 => ExchangeDataIntegrationID::NullVal,
            1 => ExchangeDataIntegrationID::BinanceData,
            _ => panic!("Invalid exchange data integration"),
        }
    }
}

impl From<ExchangeDataIntegrationID> for u32 {
    fn from(val: ExchangeDataIntegrationID) -> Self {
        val as u32
    }
}
