#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderTimeInForce {
    GoodTillCancel = 0x0_u8,
    GoodTillTimeExchange = 0x1_u8,
    FillOrKill = 0x3_u8,
    ImmediateOrCancel = 0x4_u8,
    OneCancelOther = 0x5_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for OrderTimeInForce {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::GoodTillCancel,
            0x1_u8 => Self::GoodTillTimeExchange,
            0x3_u8 => Self::FillOrKill,
            0x4_u8 => Self::ImmediateOrCancel,
            0x5_u8 => Self::OneCancelOther,
            _ => Self::NullVal,
        }
    }
}
