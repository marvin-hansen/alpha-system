use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum OrderType {
    #[default]
    Limit = 0x1_u8,
    Market = 0x2_u8,
    Stop = 0x3_u8,
    StopLimit = 0x4_u8,
}

impl From<OrderType> for u8 {
    fn from(value: OrderType) -> Self {
        value as u8
    }
}

impl From<u8> for OrderType {
    fn from(value: u8) -> Self {
        match value {
            0x1_u8 => Self::Limit,
            0x2_u8 => Self::Market,
            0x3_u8 => Self::Stop,
            0x4_u8 => Self::StopLimit,
            _ => Self::Limit,
        }
    }
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
