use crate::{FirstOHLCVBar, MessageType};

impl FirstOHLCVBar {
    #[must_use]
    pub const fn message_type(&self) -> MessageType {
        self.message_type
    }
    #[must_use]
    pub const fn symbol_id(&self) -> u16 {
        self.symbol_id
    }
}
