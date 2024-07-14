use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[repr(u8)]
pub enum ExchangeID {
    #[default]
    NullVal = 0_u8,
    Kraken = 1_u8,
    COINBASE = 2_u8,
    VEX = 3_u8,
    Binance = 4_u8,
}

impl From<i32> for ExchangeID {
    #[inline]
    fn from(v: i32) -> Self {
        match v {
            0xff_i32 => Self::NullVal,
            0x1_i32 => Self::Kraken,
            0x2_i32 => Self::COINBASE,
            0x3_i32 => Self::VEX,
            0x4_i32 => Self::Binance,
            _ => Self::NullVal,
        }
    }
}

impl From<u8> for ExchangeID {
    /// Create an ExchangeID from a u8 value.
    ///
    /// # Parameters
    ///
    /// * `v` - The u8 value to convert to an ExchangeID
    ///
    /// # Returns
    ///
    /// Returns the corresponding ExchangeID for the provided u8:
    ///
    /// - 0 -> ExchangeID::NullVal
    /// - 1 -> ExchangeID::Kraken
    ///
    /// If the u8 does not match a valid mapping, returns ExchangeID::NullVal.
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0 => Self::NullVal,
            1 => Self::Kraken,
            2 => Self::COINBASE,
            3 => Self::VEX,
            4 => Self::Binance,
            _ => Self::NullVal,
        }
    }
}

impl Display for ExchangeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
