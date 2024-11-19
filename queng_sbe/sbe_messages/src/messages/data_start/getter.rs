use crate::{DataType, MessageType, StartDataMessage};
use common_data_bar::TimeResolution;
use common_exchange::ExchangeID;

impl StartDataMessage {
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
    #[must_use]
    pub const fn time_resolution(&self) -> &TimeResolution {
        &self.time_resolution
    }
}
