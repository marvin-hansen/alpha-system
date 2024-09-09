use crate::model::exchange::Exchange;
use crate::prelude::{CreateExchange, UpdateExchange};
use common_metadata::prelude::MetaExchange;

impl Exchange {
    pub fn from_meta_exchange(meta_exchange: MetaExchange) -> Self {
        Exchange {
            exchange_id: meta_exchange.code,
            exchange_name: meta_exchange.name,
        }
    }

    pub fn to_meta_exchange(&self) -> MetaExchange {
        MetaExchange {
            code: self.exchange_id.clone(),
            name: self.exchange_name.clone(),
            kaiko_legacy_slug: String::new(), // Assuming kaiko_legacy_slug is not used
        }
    }
}

impl CreateExchange {
    pub fn from_meta_exchange(meta_exchange: MetaExchange) -> Self {
        CreateExchange {
            exchange_id: meta_exchange.code,
            exchange_name: meta_exchange.name,
        }
    }
}
impl UpdateExchange {
    pub fn from_meta_exchange(meta_exchange: MetaExchange) -> Self {
        UpdateExchange {
            exchange_name: meta_exchange.name,
        }
    }
}
