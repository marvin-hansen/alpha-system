#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderType {
    Market = 0x1_u8,
    Limit = 0x2_u8,
    Stop = 0x3_u8,
    StopLimit = 0x4_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for OrderType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Market,
            0x2_u8 => Self::Limit,
            0x3_u8 => Self::Stop,
            0x4_u8 => Self::StopLimit,
            _ => Self::NullVal,
        }
    }
}
