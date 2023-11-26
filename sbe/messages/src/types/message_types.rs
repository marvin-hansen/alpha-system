use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u8)]
pub enum MessageType {
    #[default]
    NullVal = 0xff_u8,
    StartData = 0x1_u8,
    StopData = 0x2_u8,
}

impl From<u8> for MessageType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::StartData,
            0x2_u8 => Self::StopData,
            _ => Self::NullVal,
        }
    }
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::StartData => write!(f, "StartData"),
            MessageType::StopData => write!(f, "StopData"),
            MessageType::NullVal => write!(f, "NullVal"),
        }
    }
}
