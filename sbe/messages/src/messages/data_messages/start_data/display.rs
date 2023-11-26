use crate::prelude::StartDataMessage;
use std::fmt::{Display, Formatter};

impl Display for StartDataMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "message_type: {}, exchange_id: {}, asset: {}",
            self.message_type, self.exchange_id, self.asset
        )
    }
}
