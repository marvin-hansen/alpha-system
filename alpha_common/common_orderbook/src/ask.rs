/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(Debug, Default, Eq, Clone, PartialEq)]
pub struct Ask {
    price: Decimal,
    quantity: Decimal,
}

impl Ask {
    pub fn new(price: Decimal, quantity: Decimal) -> Self {
        Self { price, quantity }
    }
}

impl Ask {
    pub fn price(&self) -> Decimal {
        self.price
    }

    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
}

impl Display for Ask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ask: price: {} quantity: {}", self.price, self.quantity)
    }
}
