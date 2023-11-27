use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u8)]
pub enum ExchangeID {
    #[default]
    NullVal = 0xff_u8,
    BNB = 0x1_u8,
    VEX = 0x2_u8,
}

impl From<u8> for ExchangeID {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0xff_u8 => Self::NullVal,
            0x1_u8 => Self::BNB,
            0x2_u8 => Self::VEX,
            _ => Self::NullVal,
        }
    }
}

impl Display for ExchangeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExchangeID::BNB => write!(f, "BNB: Binance Spot Exchange"),
            ExchangeID::VEX => write!(f, "VEX: Virtual Exchange"),
            ExchangeID::NullVal => write!(f, "NullVal"),
        }
    }
}
