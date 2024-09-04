use crate::model::exchange::PostgresExchange;
use common_metadata::prelude::MetaExchange;

impl PostgresExchange {
    pub fn from_meta_exchange(meta_exchange: MetaExchange) -> Self {
        PostgresExchange {
            exchanges_code: meta_exchange.code,
            exchanges_name: meta_exchange.name,
        }
    }

    pub fn to_meta_exchange(&self) -> MetaExchange {
        MetaExchange {
            code: self.exchanges_code.clone(),
            name: self.exchanges_name.clone(),
            kaiko_legacy_slug: String::new(), // Assuming kaiko_legacy_slug is not used
        }
    }
}
