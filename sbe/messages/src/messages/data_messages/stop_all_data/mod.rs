use serde::{Deserialize, Serialize};

use common::prelude::ExchangeID;

use crate::prelude::MessageType;

mod display;
mod getter;
mod sbe_decode;
mod sbe_encode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct StopAllDataMessage {
    message_type: MessageType,
    exchange_id: ExchangeID,
}

impl StopAllDataMessage {
    pub fn new(exchange_id: ExchangeID) -> Self {
        let message_type = MessageType::StopAllData;
        Self {
            message_type,
            exchange_id,
        }
    }
}

impl From<&[u8]> for StopAllDataMessage {
    #[inline]
    fn from(buffer: &[u8]) -> Self {
        sbe_decode::decode_stop_all_data_message(buffer)
            .expect("Failed to decode start data message")
    }
}
