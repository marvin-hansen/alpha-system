use crate::prelude::{MessageType, StartDataMessage};
use common::prelude::ExchangeID;

impl StartDataMessage {
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }
    pub fn exchange_id(&self) -> &ExchangeID {
        &self.exchange_id
    }
    pub fn asset(&self) -> &str {
        &self.asset
    }
}
