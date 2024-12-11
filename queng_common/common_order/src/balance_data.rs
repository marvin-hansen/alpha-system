use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalanceData {
    asset_id: String,
    asset_id_exchange: String,
    balance: Decimal,
    available: Decimal,
    locked: Decimal,
    traded: Decimal,
    rate_usd: Decimal,
}

impl BalanceData {
    pub fn new(
        asset_id: String,
        asset_id_exchange: String,
        balance: Decimal,
        available: Decimal,
        locked: Decimal,
        traded: Decimal,
        rate_usd: Decimal,
    ) -> Self {
        Self {
            asset_id,
            asset_id_exchange,
            balance,
            available,
            locked,
            traded,
            rate_usd,
        }
    }
}

impl BalanceData {
    pub fn asset_id(&self) -> &str {
        &self.asset_id
    }

    pub fn asset_id_exchange(&self) -> &str {
        &self.asset_id_exchange
    }

    pub fn balance(&self) -> Decimal {
        self.balance
    }

    pub fn available(&self) -> Decimal {
        self.available
    }

    pub fn locked(&self) -> Decimal {
        self.locked
    }

    pub fn traded(&self) -> Decimal {
        self.traded
    }

    pub fn rate_usd(&self) -> Decimal {
        self.rate_usd
    }
}

impl Display for BalanceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
