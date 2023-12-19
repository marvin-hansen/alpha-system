use crate::prelude::StartDataMessage;
use std::fmt::{Display, Formatter};

impl Display for StartDataMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StartDataMessage {{ message_type: {:?}, exchange_id: {:?}, symbol_id: {:?} }}",
            self.message_type, self.exchange_id, self.symbol_id
        )
    }
}
