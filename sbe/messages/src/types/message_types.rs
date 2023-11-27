use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Enum representing the different types of messages that can be sent over network.
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u8)]
pub enum MessageType {
    #[default]
    NullVal = 0xff_u8,
    StartData = 0x1_u8,
    StopData = 0x2_u8,
    StopAllData = 0x3_u8,
}

impl From<u8> for MessageType {
    /// Converts a raw byte value into a `MessageType`.
    /// Unknown message type results in NullVal
    ///
    /// # Example
    /// ```
    /// use messages::prelude::MessageType;
    ///
    /// let raw_value = 0x1;
    /// let message_type = MessageType::from(raw_value);
    ///
    /// assert_eq!(message_type, MessageType::StartData);
    /// ```
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::StartData,
            0x2_u8 => Self::StopData,
            0x3_u8 => Self::StopAllData,
            _ => Self::NullVal,
        }
    }
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::StartData => write!(f, "StartData"),
            MessageType::StopData => write!(f, "StopData"),
            MessageType::StopAllData => write!(f, "StopAllData"),
            MessageType::NullVal => write!(f, "NullVal"),
        }
    }
}
