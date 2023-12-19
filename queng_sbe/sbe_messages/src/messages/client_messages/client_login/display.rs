use std::fmt;
use crate::messages::client_messages::client_login::ClientLoginMessage;

impl fmt::Display for ClientLoginMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClientLoginMessage {{ client_id: {}, client_name: {} }}",
            self.client_id(), self.client_name())
    }
}
