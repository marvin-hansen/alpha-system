#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ClientOrderID {
    client_order_id: String,
}

impl ClientOrderID {
    /// Client Order ID is a unique identifier for an order, set by the client.
    ///
    pub fn new(client_order_id: String) -> Self {
        if client_order_id.is_empty() {
            panic!("Client Order ID is empty");
        }

        if client_order_id.len() > 10 {
            panic!("Client Order ID is too long. Max length is 10");
        }

        Self { client_order_id }
    }
}

impl ClientOrderID {
    pub fn client_order_id(&self) -> &str {
        &self.client_order_id
    }
}

impl From<String> for ClientOrderID {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<ClientOrderID> for String {
    fn from(value: ClientOrderID) -> Self {
        value.client_order_id
    }
}

impl std::fmt::Display for ClientOrderID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.client_order_id)
    }
}
