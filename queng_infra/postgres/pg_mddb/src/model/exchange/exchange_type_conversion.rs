use crate::model::exchange::Exchange;
use crate::{CreateExchange, UpdateExchange};
use common_metadata::MetaExchange;

impl Exchange {
    #[must_use]
    pub fn from_meta_exchange(meta_exchange: MetaExchange) -> Self {
        Self {
            exchange_id: meta_exchange.code.clone(),
            exchange_name: meta_exchange.name.clone(),
            exchange_hash: meta_exchange.hash(),
        }
    }

    #[must_use]
    pub fn to_meta_exchange(&self) -> MetaExchange {
        MetaExchange {
            code: self.exchange_id.clone(),
            name: self.exchange_name.clone(),
            kaiko_legacy_slug: String::new(), // Assuming kaiko_legacy_slug is not used
        }
    }
}

impl CreateExchange {
    #[must_use]
    pub fn from_meta_exchange(meta_exchange: MetaExchange) -> Self {
        Self {
            exchange_id: meta_exchange.code.clone(),
            exchange_name: meta_exchange.name.clone(),
            exchange_hash: meta_exchange.hash(),
        }
    }
}
impl UpdateExchange {
    #[must_use]
    pub fn from_meta_exchange(meta_exchange: MetaExchange) -> Self {
        Self {
            exchange_name: meta_exchange.name.clone(),
            exchange_hash: meta_exchange.hash(),
        }
    }
}
