#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ExchangeID {
    UnknownExchange = 0x0_u8,
    BNB = 0x1_u8,
    VEX = 0x2_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for ExchangeID {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::UnknownExchange,
            0x1_u8 => Self::BNB,
            0x2_u8 => Self::VEX,
            _ => Self::NullVal,
        }
    }
}
