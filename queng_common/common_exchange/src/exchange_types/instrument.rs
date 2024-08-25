use std::fmt::{Display, Formatter};

#[derive(Default, Debug, Clone, PartialEq)]
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

impl Display for Instrument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instrument: {:?}", self)
    }
}
