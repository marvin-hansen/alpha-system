use crate::messages::client_messages::client_login::ClientLoginMessage;
use crate::prelude::MessageType;

impl ClientLoginMessage {
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }
    pub fn client_id(&self) -> u16 {
        self.client_id
    }
    pub fn client_name(&self) -> String {
       String::from_utf8(self.client_name.to_vec()).unwrap()
    }
}
