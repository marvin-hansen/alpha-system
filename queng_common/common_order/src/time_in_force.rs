use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum TimeInForce {
    #[default]
    GoodTillCancel = 0x1_u8,
    GoodTillTimeExchange = 0x2_u8,
    FillOrKill = 0x3_u8,
    ImmediateOrCancel = 0x4_u8,
    OneCancelOther = 0x5_u8,
}

impl From<TimeInForce> for u8 {
    #[inline]
    fn from(value: TimeInForce) -> Self {
        value as u8
    }
}

impl From<&TimeInForce> for u8 {
    #[inline]
    fn from(value: &TimeInForce) -> Self {
        value.to_owned() as u8
    }
}

impl From<TimeInForce> for u16 {
    #[inline]
    fn from(value: TimeInForce) -> Self {
        value as u16
    }
}

impl From<TimeInForce> for u32 {
    #[inline]
    fn from(value: TimeInForce) -> Self {
        value as u32
    }
}

impl From<u8> for TimeInForce {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0x1_u8 => Self::GoodTillCancel,
            0x2_u8 => Self::GoodTillTimeExchange,
            0x3_u8 => Self::FillOrKill,
            0x4_u8 => Self::ImmediateOrCancel,
            0x5_u8 => Self::OneCancelOther,
            _ => Self::GoodTillCancel,
        }
    }
}

impl From<u16> for TimeInForce {
    #[inline]
    fn from(value: u16) -> Self {
        match value {
            0x1_u16 => Self::GoodTillCancel,
            0x2_u16 => Self::GoodTillTimeExchange,
            0x3_u16 => Self::FillOrKill,
            0x4_u16 => Self::ImmediateOrCancel,
            0x5_u16 => Self::OneCancelOther,
            _ => Self::GoodTillCancel,
        }
    }
}
impl fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
