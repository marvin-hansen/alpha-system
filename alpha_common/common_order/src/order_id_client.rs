/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use encoding_utils::{decode_int64_to_str, encode_str_to_int64};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ClientOrderID {
    client_order_id: String,
    client_order_id_binary: u64,
}

impl ClientOrderID {
    /// Client Order ID is a unique identifier for an order, set by the client.
    ///
    pub fn new(client_order_id: &str) -> Self {
        if client_order_id.is_empty() {
            panic!("Client Order ID is empty");
        }

        if client_order_id.len() > 10 {
            panic!("Client Order ID is too long. Max length is 10");
        }

        let client_order_id_binary =
            encode_str_to_int64(client_order_id).expect("Failed to encode client order ID");

        Self {
            client_order_id: client_order_id.to_string(),
            client_order_id_binary,
        }
    }
}

impl ClientOrderID {
    pub fn client_order_id(&self) -> &str {
        &self.client_order_id
    }

    pub fn client_order_id_binary(&self) -> u64 {
        self.client_order_id_binary
    }
}

impl From<String> for ClientOrderID {
    fn from(value: String) -> Self {
        Self::new(value.as_str())
    }
}

impl From<&str> for ClientOrderID {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<ClientOrderID> for String {
    fn from(value: ClientOrderID) -> Self {
        value.client_order_id
    }
}

impl From<ClientOrderID> for u64 {
    fn from(value: ClientOrderID) -> Self {
        value.client_order_id_binary
    }
}

impl From<u64> for ClientOrderID {
    fn from(value: u64) -> Self {
        let value = decode_int64_to_str(value).expect("Failed to decode client order ID");
        Self::new(&value)
    }
}

impl std::fmt::Display for ClientOrderID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.client_order_id)
    }
}
