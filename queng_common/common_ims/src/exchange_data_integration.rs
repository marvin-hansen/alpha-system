use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ExchangeDataIntegrationID {
    BinanceData = 0,
    MockData = 1,
    VexData = 2,
}

impl Display for ExchangeDataIntegrationID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<u8> for ExchangeDataIntegrationID {
    fn from(value: u8) -> Self {
        match value {
            0 => ExchangeDataIntegrationID::BinanceData,
            1 => ExchangeDataIntegrationID::MockData,
            2 => ExchangeDataIntegrationID::VexData,
            _ => panic!("Invalid exchange data integration"),
        }
    }
}

impl Into<u8> for ExchangeDataIntegrationID {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u16> for ExchangeDataIntegrationID {
    fn from(value: u16) -> Self {
        match value {
            0 => ExchangeDataIntegrationID::BinanceData,
            1 => ExchangeDataIntegrationID::MockData,
            2 => ExchangeDataIntegrationID::VexData,
            _ => panic!("Invalid exchange data integration"),
        }
    }
}

impl Into<u16> for ExchangeDataIntegrationID {
    fn into(self) -> u16 {
        self as u16
    }
}

impl From<u32> for ExchangeDataIntegrationID {
    fn from(value: u32) -> Self {
        match value {
            0 => ExchangeDataIntegrationID::BinanceData,
            1 => ExchangeDataIntegrationID::MockData,
            2 => ExchangeDataIntegrationID::VexData,
            _ => panic!("Invalid exchange data integration"),
        }
    }
}

impl Into<u32> for ExchangeDataIntegrationID {
    fn into(self) -> u32 {
        self as u32
    }
}
