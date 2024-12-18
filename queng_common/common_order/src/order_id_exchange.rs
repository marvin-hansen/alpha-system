use encoding_utils::encode_str_to_pair_u64;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ExchangeOrderID {
    exchange_order_id: String,
    exchange_order_id_binary: (u64, u64),
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

        let exchange_order_id_binary =
            encode_str_to_pair_u64(&exchange_order_id).expect("Failed to encode Exchange Order ID");

        Self {
            exchange_order_id,
            exchange_order_id_binary,
        }
    }
}

impl ExchangeOrderID {
    pub fn exchange_order_id(&self) -> &str {
        &self.exchange_order_id
    }

    pub fn exchange_order_id_binary(&self) -> &(u64, u64) {
        &self.exchange_order_id_binary
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
