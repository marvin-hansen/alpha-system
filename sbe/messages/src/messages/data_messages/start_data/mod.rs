use crate::prelude::MessageType;
use common::prelude::ExchangeID;
use serde::{Deserialize, Serialize};

mod display;
mod getter;
mod sbe_decode;
mod sbe_encode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct StartDataMessage {
    message_type: MessageType,
    exchange_id: ExchangeID,
    asset: String,
}

impl StartDataMessage {
    pub fn new(exchange_id: ExchangeID, asset: String) -> Self {
        Self {
            message_type: MessageType::StartData,
            exchange_id,
            asset,
        }
    }
}

impl From<&[u8]> for StartDataMessage {
    #[inline]
    fn from(buffer: &[u8]) -> Self {
        sbe_decode::decode_start_data_message(buffer).expect("Failed to decode start data message")
    }
}
