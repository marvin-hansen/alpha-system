use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instrument {
    code: String,
    class: String,
    exchange_code: String,
    exchange_pair_code: String,
    base_asset: String,
    quote_asset: String,
    instrument_figi: Option<String>,
}

impl Instrument {
    pub fn new(
        code: String,
        class: String,
        exchange_code: String,
        exchange_pair_code: String,
        base_asset: String,
        quote_asset: String,
        instrument_figi: Option<String>,
    ) -> Self {
        Self {
            code,
            class,
            exchange_code,
            exchange_pair_code,
            base_asset,
            quote_asset,
            instrument_figi,
        }
    }
}

impl Instrument {
    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn class(&self) -> &str {
        &self.class
    }

    pub fn exchange_code(&self) -> &str {
        &self.exchange_code
    }

    pub fn exchange_pair_code(&self) -> &str {
        &self.exchange_pair_code
    }

    pub fn base_asset(&self) -> &str {
        &self.base_asset
    }

    pub fn quote_asset(&self) -> &str {
        &self.quote_asset
    }

    pub fn instrument_figi(&self) -> &Option<String> {
        &self.instrument_figi
    }
}

impl Instrument {
    pub fn from_sql_row(row: &Row) -> Self {
        // let id = row.get::<usize, i32>(0); // PK
        let code = row.get::<usize, String>(1);
        let class = row.get::<usize, String>(2);
        let exchange_code = row.get::<usize, String>(3);
        let exchange_pair_code = row.get::<usize, String>(4);
        let base_asset = row.get::<usize, String>(5);
        let quote_asset = row.get::<usize, String>(6);
        let instrument_figi = row.get::<usize, Option<String>>(7);
        Instrument::new(
            code,
            class,
            exchange_code,
            exchange_pair_code,
            base_asset,
            quote_asset,
            instrument_figi,
        )
    }
}

impl Display for Instrument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instrument: {:?}", self)
    }
}
