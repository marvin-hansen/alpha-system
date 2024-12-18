use encoding_utils::{decode_pair_64_to_str, encode_str_to_pair_u64};
use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct OrderExchangeSymbol {
    exchange_symbol_id: String,
    exchange_symbol_id_binary: (u64, u64),
}

impl OrderExchangeSymbol {
    pub fn new(exchange_symbol_id: &str) -> Self {
        println!("exchange_symbol_id: {}", exchange_symbol_id);

        if exchange_symbol_id.is_empty() {
            panic!("Exchange Symbol is empty");
        }

        if exchange_symbol_id.len() > 20 {
            panic!("Exchange Symbol is too long. Max length is 20");
        }

        let exchange_symbol_id_binary =
            encode_str_to_pair_u64(exchange_symbol_id).expect("Failed to encode Exchange Order ID");

        println!(
            "exchange_symbol_id_binary: ({}, {})",
            exchange_symbol_id_binary.0, exchange_symbol_id_binary.1
        );

        Self {
            exchange_symbol_id: exchange_symbol_id.to_string(),
            exchange_symbol_id_binary,
        }
    }
}

impl OrderExchangeSymbol {
    pub fn exchange_order_id(&self) -> &str {
        &self.exchange_symbol_id
    }

    pub fn exchange_order_id_binary(&self) -> (u64, u64) {
        self.exchange_symbol_id_binary
    }
}

impl From<String> for OrderExchangeSymbol {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}

impl From<&str> for OrderExchangeSymbol {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<(u64, u64)> for OrderExchangeSymbol {
    fn from(value: (u64, u64)) -> Self {
        println!("exchange_symbol_id_binary: ({}, {})", value.0, value.1);

        let exchange_symbol_id =
            decode_pair_64_to_str(value).expect("Failed to decode Exchange Order ID");

        println!("Decoded exchange_symbol_id: {}", exchange_symbol_id);

        Self::new(&exchange_symbol_id)
    }
}

impl Display for OrderExchangeSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.exchange_symbol_id)
    }
}
