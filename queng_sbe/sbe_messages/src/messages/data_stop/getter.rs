use crate::messages::data_stop::StopDataMessage;
use crate::{DataType, MessageType};
use common_exchange::ExchangeID;

impl StopDataMessage {
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
    #[must_use]
    pub const fn symbol_id(&self) -> &u16 {
        &self.symbol_id
    }

    #[must_use]
    pub const fn data_type_id(&self) -> &DataType {
        &self.data_type_id
    }
}
