use crate::prelude::MessageType;
use common::prelude::MessageClientConfig;
use serde::{Deserialize, Serialize};

mod display;
mod getters;
mod sbe_decode;
mod sbe_encode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct ClientLoginMessage {
    message_type: MessageType,
    client_id: u16,
    client_name: [u8; 10],
}

impl ClientLoginMessage {
    pub fn from_config(conf: &MessageClientConfig) -> Self {
        let client_id = conf.id();
        let client_name = conf.name().to_string().as_bytes().try_into().unwrap();
        let message_type = MessageType::ClientLogin;

        Self {
            message_type,
            client_id,
            client_name,
        }
    }

    pub fn new(client_id: u16, client_name: [u8; 10]) -> Self {
        let message_type = MessageType::ClientLogin;

        Self {
            message_type,
            client_id,
            client_name,
        }
    }
}

impl From<&[u8]> for ClientLoginMessage {
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_client_login_message(value).expect("Failed to decode ClientLoginMessage")
    }
}
