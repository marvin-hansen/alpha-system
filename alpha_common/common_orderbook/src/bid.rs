/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(Debug, Default, Eq, Clone, PartialEq)]
pub struct Bid {
    price: Decimal,
    quantity: Decimal,
}

impl Bid {
    pub fn new(price: Decimal, quantity: Decimal) -> Self {
        Self { price, quantity }
    }
}

impl Bid {
    pub fn price(&self) -> Decimal {
        self.price
    }

    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
}

impl Display for Bid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bid: price: {} quantity: {}", self.price, self.quantity)
    }
}
