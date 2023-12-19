use std::fmt;
use crate::prelude::ClientLogoutMessage;

impl fmt::Display for ClientLogoutMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClientLogoutMessage {{ client_id: {} }}", self.client_id())
    }
}
