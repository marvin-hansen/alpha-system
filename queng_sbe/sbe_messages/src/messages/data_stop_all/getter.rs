use crate::messages::data_stop_all::StopAllDataMessage;
use crate::MessageType;
use common_exchange::ExchangeID;

impl StopAllDataMessage {
    #[must_use]
    pub const fn message_type(&self) -> &MessageType {
        &self.message_type
    }
    #[must_use]
    pub const fn client_id(&self) -> &u16 {
        &self.client_id
    }
    #[must_use]
    pub const fn exchange_id(&self) -> &ExchangeID {
        &self.exchange_id
    }
}
