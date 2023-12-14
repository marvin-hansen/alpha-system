use crate::messages::data_messages::stop_data::StopDataMessage;
use std::fmt::{Display, Formatter};

impl Display for StopDataMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StopDataMessage {{ message_type: {:?}, exchange_id: {:?}, symbol_id: {:?} }}",
            self.message_type, self.exchange_id, self.symbol_id
        )
    }
}
