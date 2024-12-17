#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ExchangeOrderID {
    exchange_order_id: String,
}

impl ExchangeOrderID {
    /// Creates a new ExchangeOrderID instance.
    ///
    /// # Panics
    /// If exchange_order_id is empty or longer than 20 characters.
    pub fn new(exchange_order_id: String) -> Self {
        if exchange_order_id.is_empty() {
            panic!("Exchange Order ID is empty");
        }

        if exchange_order_id.len() > 20 {
            panic!("Exchange Order ID is too long. Max length is 20");
        }

        Self { exchange_order_id }
    }
}

impl ExchangeOrderID {
    pub fn exchange_order_id(&self) -> &str {
        &self.exchange_order_id
    }
}

impl From<String> for ExchangeOrderID {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for ExchangeOrderID {
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

impl From<ExchangeOrderID> for String {
    fn from(value: ExchangeOrderID) -> Self {
        value.exchange_order_id
    }
}

impl std::fmt::Display for ExchangeOrderID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.exchange_order_id)
    }
}
