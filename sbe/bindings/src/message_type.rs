#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MessageType {
    UnknownMessageType = 0x0_u8,
    StartData = 0x1_u8,
    StopData = 0x2_u8,
    StopAllData = 0x3_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for MessageType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::UnknownMessageType,
            0x1_u8 => Self::StartData,
            0x2_u8 => Self::StopData,
            0x3_u8 => Self::StopAllData,
            _ => Self::NullVal,
        }
    }
}
