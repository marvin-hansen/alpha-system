use std::fmt::{Display, Formatter};
use crate::messages::data_messages::stop_all_data::StopAllDataMessage;

impl Display for StopAllDataMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "StopAllDataMessage {{ message_type: {:?}, exchange_id: {:?} }}",
               self.message_type, self.exchange_id,
        )
    }
}
