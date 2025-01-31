/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::balance_data::BalanceData;
use common_exchange::ExchangeID;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Balances {
    exchange_id: ExchangeID,
    balances: Vec<BalanceData>,
}

impl Balances {
    pub fn new(exchange_id: ExchangeID, balances: Vec<BalanceData>) -> Self {
        Self {
            exchange_id,
            balances,
        }
    }
}

impl Balances {
    pub fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }

    pub fn balances(&self) -> &Vec<BalanceData> {
        &self.balances
    }
}

impl Display for Balances {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
